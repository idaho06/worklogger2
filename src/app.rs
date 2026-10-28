use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwg::NativeUi;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use crate::config::Config;
use crate::file_ops;
use crate::dialogs::{InputDialog, AboutDialog};

// Theme colors (BGR format for Windows GDI)
const DARK_BG: u32 = 0x00282828;      // Dark gray background
const DARK_FG: u32 = 0x00E0E0E0;      // Light gray text
const LIGHT_BG: u32 = 0x00FFFFFF;     // White background
const LIGHT_FG: u32 = 0x00000000;     // Black text

/// Holds theme-related state shared with raw event handlers
struct ThemeState {
    is_dark: Arc<AtomicBool>,
}

/// Main application state and UI
#[derive(Default, nwd::NwgUi)]
pub struct WorkLoggerApp {
    // ===== Application Icon =====
    #[nwg_resource(source_embed: Some(&nwg::EmbedResource::load(None).unwrap()), source_embed_id: 1)]
    app_icon: nwg::Icon,

    // ===== Main Window =====
    #[nwg_control(
        size: (800, 600),
        position: (300, 150),
        title: "WorkLogger",
        icon: Some(&data.app_icon),
        flags: "WINDOW|VISIBLE|MINIMIZE_BOX|MAXIMIZE_BOX|RESIZABLE"
    )]
    #[nwg_events(
        OnWindowClose: [WorkLoggerApp::on_close],
        OnInit: [WorkLoggerApp::on_init],
        OnResizeEnd: [WorkLoggerApp::on_resize],
        OnWindowMaximize: [WorkLoggerApp::on_resize]
    )]
    pub window: nwg::Window,

    // ===== Text Editor =====
    #[nwg_control(
        parent: window,
        flags: "VISIBLE|AUTOVSCROLL|VSCROLL|HSCROLL|TAB_STOP",
        font: Some(&data.editor_font)
    )]
    pub text_edit: nwg::TextBox,

    // Font for the text editor
    #[nwg_resource(family: "Consolas", size: 18, weight: 400)]
    editor_font: nwg::Font,

    // ===== Menu Bar =====

    // -- Action Menu --
    #[nwg_control(parent: window, text: "&Action")]
    menu_action: nwg::Menu,

    #[nwg_control(parent: menu_action, text: "&Quit\tCtrl+Q")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_quit])]
    menu_quit: nwg::MenuItem,

    // -- Insert Menu --
    #[nwg_control(parent: window, text: "&Insert")]
    menu_insert: nwg::Menu,

    #[nwg_control(parent: menu_insert, text: "&Time hh:mm\tCtrl+T")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_insert_time])]
    menu_time: nwg::MenuItem,

    // -- Options Menu --
    #[nwg_control(parent: window, text: "&Options")]
    menu_options: nwg::Menu,

    #[nwg_control(parent: menu_options, text: "Change &Theme\tCtrl+D")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_toggle_theme])]
    menu_theme: nwg::MenuItem,

    #[nwg_control(parent: menu_options, text: "&Insert Time At Start")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_toggle_insert_time])]
    menu_insert_time_start: nwg::MenuItem,

    #[nwg_control(parent: menu_options, text: "&Word Wrap")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_toggle_word_wrap])]
    menu_word_wrap: nwg::MenuItem,

    #[nwg_control(parent: menu_options)]
    menu_options_sep: nwg::MenuSeparator,

    #[nwg_control(parent: menu_options, text: "&Log Directory...")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_choose_directory])]
    menu_log_dir: nwg::MenuItem,

    #[nwg_control(parent: menu_options, text: "&Project Name...")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_choose_project_name])]
    menu_project_name: nwg::MenuItem,

    // -- Help Menu --
    #[nwg_control(parent: window, text: "&Help")]
    menu_help: nwg::Menu,

    #[nwg_control(parent: menu_help, text: "&About WorkLogger...\tF1")]
    #[nwg_events(OnMenuItemSelected: [WorkLoggerApp::on_about])]
    menu_about: nwg::MenuItem,

    // ===== Dialogs =====
    dir_dialog: RefCell<nwg::FileDialog>,

    // ===== Application State =====
    config: RefCell<Config>,
    /// Text present right after startup (the auto-inserted time, if any);
    /// an editor still holding only this is not worth logging
    initial_text: RefCell<String>,
    theme_state: RefCell<Option<ThemeState>>,
}

impl WorkLoggerApp {
    /// Build and return the UI with keyboard accelerators
    pub fn build() -> Result<Rc<WorkLoggerAppUi>, nwg::NwgError> {
        // Build the base UI
        let app_ui = WorkLoggerApp::build_ui(Default::default())?;

        // Set up keyboard accelerator handler
        let app_rc = Rc::new(app_ui);

        // Clone for the event handler
        let app_weak = Rc::downgrade(&app_rc);

        // Create brushes for themes (these are leaked intentionally to live for app lifetime)
        let dark_brush = unsafe { winapi::um::wingdi::CreateSolidBrush(DARK_BG) };
        let light_brush = unsafe { winapi::um::wingdi::CreateSolidBrush(LIGHT_BG) };

        // Shared theme state for the handler
        let is_dark = Arc::new(AtomicBool::new(true));
        let is_dark_for_handler = Arc::clone(&is_dark);

        // Store in app for later use
        *app_rc.theme_state.borrow_mut() = Some(ThemeState {
            is_dark,
        });

        // Bind raw event handlers to catch keyboard and color messages
        let make_handler = move |app_weak: Weak<WorkLoggerAppUi>, is_dark: Arc<AtomicBool>, dark_brush: usize, light_brush: usize| {
            move |_hwnd, msg, wparam, _lparam| {
                const WM_KEYDOWN: u32 = 0x0100;
                const WM_CTLCOLOREDIT: u32 = 0x0133;

                match msg {
                    WM_KEYDOWN => {
                        if let Some(app) = app_weak.upgrade() {
                            let ctrl_pressed = unsafe {
                                winapi::um::winuser::GetKeyState(
                                    winapi::um::winuser::VK_CONTROL
                                ) < 0
                            };

                            let key = wparam as i32;

                            match (key, ctrl_pressed) {
                                // Ctrl+T - Insert time
                                (0x54, true) => {  // 'T' key
                                    app.insert_time_at_cursor();
                                    return Some(0);
                                }
                                // Ctrl+Q - Quit
                                (0x51, true) => {  // 'Q' key
                                    app.on_close();
                                    return Some(0);
                                }
                                // Ctrl+D - Toggle theme
                                (0x44, true) => {  // 'D' key
                                    app.on_toggle_theme();
                                    return Some(0);
                                }
                                // F1 - About
                                (0x70, false) => {  // VK_F1
                                    app.on_about();
                                    return Some(0);
                                }
                                // ESC - Close
                                (0x1B, false) => {  // VK_ESCAPE
                                    app.on_close();
                                    return Some(0);
                                }
                                _ => {}
                            }
                        }
                    }
                    WM_CTLCOLOREDIT => {
                        let hdc = wparam as winapi::shared::windef::HDC;
                        let dark = is_dark.load(Ordering::Relaxed);

                        unsafe {
                            if dark {
                                winapi::um::wingdi::SetTextColor(hdc, DARK_FG);
                                winapi::um::wingdi::SetBkColor(hdc, DARK_BG);
                                return Some(dark_brush as isize);
                            } else {
                                winapi::um::wingdi::SetTextColor(hdc, LIGHT_FG);
                                winapi::um::wingdi::SetBkColor(hdc, LIGHT_BG);
                                return Some(light_brush as isize);
                            }
                        }
                    }
                    _ => {}
                }
                None
            }
        };

        let handler_window = nwg::bind_raw_event_handler(
            &app_rc.window.handle,
            0x10000,  // Handler ID (window)
            make_handler(app_weak.clone(), Arc::clone(&is_dark_for_handler), dark_brush as usize, light_brush as usize)
        );

        let handler_text = nwg::bind_raw_event_handler(
            &app_rc.text_edit.handle,
            0x10001,  // Handler ID (text box)
            make_handler(app_weak, is_dark_for_handler, dark_brush as usize, light_brush as usize)
        );

        // Store handlers to prevent them from being dropped
        std::mem::forget(handler_window);
        std::mem::forget(handler_text);

        // Return the Rc - this keeps it alive so the Weak reference in the handler works
        Ok(app_rc)
    }

    /// Called when window initializes
    fn on_init(&self) {
        // Initialize FileDialog
        let mut dialog = nwg::FileDialog::default();
        nwg::FileDialog::builder()
            .title("Select Log Directory")
            .action(nwg::FileDialogAction::OpenDirectory)
            .build(&mut dialog)
            .expect("Failed to build file dialog");
        *self.dir_dialog.borrow_mut() = dialog;

        // Load config
        let config = Config::load();
        *self.config.borrow_mut() = config.clone();

        // Apply theme
        self.apply_theme();

        // Set checkbox states for menu items
        self.menu_insert_time_start.set_checked(config.insert_time_at_start);
        self.menu_word_wrap.set_checked(config.word_wrap);

        // Insert time if configured
        if config.insert_time_at_start {
            self.insert_time_at_cursor();
        }
        *self.initial_text.borrow_mut() = self.text_edit.text();

        // Set focus to text editor
        self.text_edit.set_focus();

        // Initial resize to position text editor
        self.on_resize();
    }

    /// Called when window is resized
    fn on_resize(&self) {
        let (w, h) = self.window.size();
        self.text_edit.set_position(0, 0);
        self.text_edit.set_size(w, h);
    }

    /// Called when window close is requested
    fn on_close(&self) {
        if self.save_text() {
            nwg::stop_thread_dispatch();
        }
    }

    // ===== Action Menu Handlers =====

    fn on_quit(&self) {
        self.on_close();
    }

    // ===== Insert Menu Handlers =====

    fn on_insert_time(&self) {
        self.insert_time_at_cursor();
    }

    /// Insert current time (HH:MM) at cursor position
    fn insert_time_at_cursor(&self) {
        let time_str = file_ops::get_current_time();

        // Let the edit control replace its own selection: it keeps the undo
        // history and scroll position, and avoids converting selection offsets
        // (UTF-16 units, CRLF line breaks) to indexes into the text we read back.
        if let nwg::ControlHandle::Hwnd(hwnd) = self.text_edit.handle {
            let wide: Vec<u16> = time_str.encode_utf16().chain(std::iter::once(0)).collect();
            unsafe {
                winapi::um::winuser::SendMessageW(
                    hwnd,
                    winapi::um::winuser::EM_REPLACESEL as u32,
                    1, // can be undone
                    wide.as_ptr() as winapi::shared::minwindef::LPARAM,
                );
            }
        }
    }

    // ===== Options Menu Handlers =====

    fn on_toggle_theme(&self) {
        self.config.borrow_mut().toggle_theme();
        self.apply_theme();
    }

    fn on_toggle_insert_time(&self) {
        let mut config = self.config.borrow_mut();
        config.toggle_insert_time();
        self.menu_insert_time_start.set_checked(config.insert_time_at_start);
    }

    fn on_toggle_word_wrap(&self) {
        let mut config = self.config.borrow_mut();
        config.toggle_word_wrap();
        self.menu_word_wrap.set_checked(config.word_wrap);

        nwg::simple_message(
            "Word Wrap",
            "Word wrap setting saved.\nRestart the application for changes to take effect."
        );
    }

    fn on_choose_directory(&self) {
        let dialog = self.dir_dialog.borrow();
        if dialog.run(Some(&self.window)) {
            if let Ok(path) = dialog.get_selected_item() {
                let path_buf: PathBuf = path.into();
                self.config.borrow_mut().set_log_path(path_buf.clone());

                nwg::simple_message(
                    "Log Directory",
                    &format!("Log directory set to:\n{}", path_buf.display())
                );
            }
        }
    }

    fn on_choose_project_name(&self) {
        let current_name = self.config.borrow().project_name.clone();

        if let Some(new_name) = InputDialog::get_input(
            &self.window,
            "Project Name",
            "Enter project name (prefix for log files):",
            &current_name
        ) {
            let trimmed = new_name.trim();
            if !trimmed.is_empty() && trimmed != current_name {
                let stored = {
                    let mut config = self.config.borrow_mut();
                    config.set_project_name(trimmed);
                    config.project_name.clone()
                };

                nwg::simple_message(
                    "Project Name",
                    &format!(
                        "Project name changed to: {}\n\nLog files will be named: {}YYYYMMDD.txt",
                        stored, stored
                    )
                );
            }
        }
    }

    // ===== Help Menu Handlers =====

    fn on_about(&self) {
        AboutDialog::show(&self.window);
    }

    // ===== Theme Handling =====

    fn apply_theme(&self) {
        let is_dark = self.config.borrow().is_dark_theme();

        // Update the atomic flag for the raw event handler
        if let Some(ref theme_state) = *self.theme_state.borrow() {
            theme_state.is_dark.store(is_dark, Ordering::Relaxed);
        }


        // Force repaint of the text editor to apply new colors
        if let nwg::ControlHandle::Hwnd(hwnd) = self.text_edit.handle {
            unsafe {
                winapi::um::winuser::InvalidateRect(hwnd, std::ptr::null(), 1);
            }
        }
    }

    // ===== File Operations =====

    /// Save the editor contents to today's log file.
    /// Returns false if saving failed and the user chose to keep the window open.
    fn save_text(&self) -> bool {
        let content = self.text_edit.text();

        if content.trim().is_empty() || content.trim() == self.initial_text.borrow().trim() {
            return true;
        }

        let result = {
            let config = self.config.borrow();
            file_ops::save_log(&config.log_path, &config.project_name, &content)
        };

        match result {
            Ok(()) => true,
            Err(e) => {
                let choice = nwg::message(&nwg::MessageParams {
                    title: "Save Error",
                    content: &format!(
                        "Failed to save log: {}\n\nQuit without saving? Choose No to keep the window open.",
                        e
                    ),
                    buttons: nwg::MessageButtons::YesNo,
                    icons: nwg::MessageIcons::Error,
                });
                choice == nwg::MessageChoice::Yes
            }
        }
    }
}

/// Type alias for the UI wrapper
pub type WorkLoggerAppUi = work_logger_app_ui::WorkLoggerAppUi;
