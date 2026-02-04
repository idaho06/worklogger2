use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwg::NativeUi;

use std::cell::RefCell;

// ============================================================================
// Input Dialog
// ============================================================================

/// Input dialog for getting text input from user
#[derive(Default, nwd::NwgUi)]
pub struct InputDialog {
    #[nwg_control(
        size: (400, 140),
        position: (400, 300),
        title: "Input",
        flags: "WINDOW|VISIBLE",
        topmost: true
    )]
    #[nwg_events(
        OnWindowClose: [InputDialog::on_cancel],
        OnInit: [InputDialog::on_init]
    )]
    pub window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 16)]
    label_font: nwg::Font,

    #[nwg_control(
        text: "Enter value:",
        position: (15, 15),
        size: (370, 22),
        font: Some(&data.label_font)
    )]
    label: nwg::Label,

    #[nwg_control(
        text: "",
        position: (15, 42),
        size: (370, 28),
        flags: "VISIBLE|TAB_STOP"
    )]
    pub input: nwg::TextInput,

    #[nwg_control(text: "OK", position: (210, 85), size: (85, 35))]
    #[nwg_events(OnButtonClick: [InputDialog::on_ok])]
    btn_ok: nwg::Button,

    #[nwg_control(text: "Cancel", position: (300, 85), size: (85, 35))]
    #[nwg_events(OnButtonClick: [InputDialog::on_cancel])]
    btn_cancel: nwg::Button,

    /// The label text to show
    label_text: RefCell<String>,

    /// The initial value
    initial_value: RefCell<String>,

    /// Result value - Some(text) if OK pressed, None if cancelled
    pub result: RefCell<Option<String>>,
}

impl InputDialog {
    /// Show the input dialog and get user input
    /// Returns Some(input) if OK was pressed, None if cancelled
    pub fn get_input(parent: &nwg::Window, title: &str, label: &str, current: &str) -> Option<String> {
        let dialog = InputDialog::build_ui(Default::default())
            .expect("Failed to build input dialog");

        dialog.window.set_text(title);
        *dialog.label_text.borrow_mut() = label.to_string();
        *dialog.initial_value.borrow_mut() = current.to_string();

        // Center on parent
        let (px, py) = parent.position();
        let (pw, ph) = parent.size();
        let (dw, dh) = dialog.window.size();
        dialog.window.set_position(
            px + (pw as i32 - dw as i32) / 2,
            py + (ph as i32 - dh as i32) / 2
        );

        nwg::dispatch_thread_events();

        let result = dialog.result.borrow().clone();
        result
    }

    fn on_init(&self) {
        self.label.set_text(&self.label_text.borrow());
        self.input.set_text(&self.initial_value.borrow());
        self.input.set_focus();

        // Select all text for easy replacement
        let len = self.input.text().len() as u32;
        self.input.set_selection(0..len);
    }

    fn on_ok(&self) {
        let text = self.input.text();
        *self.result.borrow_mut() = Some(text);
        self.close();
    }

    fn on_cancel(&self) {
        *self.result.borrow_mut() = None;
        self.close();
    }

    fn close(&self) {
        self.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }
}

// ============================================================================
// About Dialog
// ============================================================================

/// About dialog with application information
#[derive(Default, nwd::NwgUi)]
pub struct AboutDialog {
    #[nwg_control(
        size: (450, 280),
        position: (400, 250),
        title: "About WorkLogger",
        flags: "WINDOW|VISIBLE",
        topmost: true
    )]
    #[nwg_events(
        OnWindowClose: [AboutDialog::on_close],
        OnInit: [AboutDialog::on_init]
    )]
    pub window: nwg::Window,

    // Fonts
    #[nwg_resource(family: "Segoe UI", size: 24, weight: 700)]
    title_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 15)]
    info_font: nwg::Font,

    #[nwg_resource(family: "Segoe UI", size: 13)]
    small_font: nwg::Font,

    // Application name and version
    #[nwg_control(
        text: "WorkLogger",
        position: (20, 15),
        size: (410, 35),
        font: Some(&data.title_font)
    )]
    title_label: nwg::Label,

    #[nwg_control(
        text: "Version 2.0.0",
        position: (20, 50),
        size: (200, 20),
        font: Some(&data.info_font)
    )]
    version_label: nwg::Label,

    // Description
    #[nwg_control(
        text: "A text editor for creating daily work log files.\nSimple, fast, and focused on one task.",
        position: (20, 80),
        size: (410, 45),
        font: Some(&data.info_font)
    )]
    desc_label: nwg::Label,

    // Separator frame
    #[nwg_control(
        flags: "VISIBLE",
        position: (20, 130),
        size: (410, 2)
    )]
    separator: nwg::Frame,

    // Author info
    #[nwg_control(
        text: "Author:      idaho06 (AkinoSoft)\n\
               Email:        cesar.idaho@gmail.com\n\
               GitHub:      github.com/idaho06\n\
               Mastodon:     @idaho06@mastodon.social",
        position: (20, 145),
        size: (410, 80),
        font: Some(&data.info_font)
    )]
    author_label: nwg::Label,

    // Copyright
    #[nwg_control(
        text: "Copyright \u{00A9} 2024-2026 idaho06. All rights reserved.",
        position: (20, 230),
        size: (300, 20),
        font: Some(&data.small_font)
    )]
    copyright_label: nwg::Label,

    // OK button
    #[nwg_control(text: "OK", position: (350, 225), size: (80, 35))]
    #[nwg_events(OnButtonClick: [AboutDialog::on_close])]
    btn_ok: nwg::Button,
}

impl AboutDialog {
    /// Show the About dialog centered on parent window
    pub fn show(parent: &nwg::Window) {
        let dialog = AboutDialog::build_ui(Default::default())
            .expect("Failed to build About dialog");

        // Center on parent
        let (px, py) = parent.position();
        let (pw, ph) = parent.size();
        let (dw, dh) = dialog.window.size();
        dialog.window.set_position(
            px + (pw as i32 - dw as i32) / 2,
            py + (ph as i32 - dh as i32) / 2
        );

        nwg::dispatch_thread_events();
    }

    fn on_init(&self) {
        self.btn_ok.set_focus();
    }

    fn on_close(&self) {
        self.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }
}

// ============================================================================
// Confirmation Dialog
// ============================================================================

/// Confirmation dialog for yes/no questions
#[derive(Default, nwd::NwgUi)]
pub struct ConfirmDialog {
    #[nwg_control(
        size: (400, 140),
        position: (400, 300),
        title: "Confirm",
        flags: "WINDOW|VISIBLE",
        topmost: true
    )]
    #[nwg_events(
        OnWindowClose: [ConfirmDialog::on_no],
        OnInit: [ConfirmDialog::on_init]
    )]
    pub window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 16)]
    message_font: nwg::Font,

    #[nwg_control(
        text: "Are you sure?",
        position: (15, 15),
        size: (370, 55),
        font: Some(&data.message_font)
    )]
    message_label: nwg::Label,

    #[nwg_control(text: "Yes", position: (210, 85), size: (85, 35))]
    #[nwg_events(OnButtonClick: [ConfirmDialog::on_yes])]
    btn_yes: nwg::Button,

    #[nwg_control(text: "No", position: (300, 85), size: (85, 35))]
    #[nwg_events(OnButtonClick: [ConfirmDialog::on_no])]
    btn_no: nwg::Button,

    /// The message to display
    message: RefCell<String>,

    /// Result - true if Yes, false if No
    pub result: RefCell<bool>,
}

impl ConfirmDialog {
    /// Show confirmation dialog
    /// Returns true if user clicked Yes, false otherwise
    #[allow(dead_code)]
    pub fn confirm(parent: &nwg::Window, title: &str, message: &str) -> bool {
        let dialog = ConfirmDialog::build_ui(Default::default())
            .expect("Failed to build confirm dialog");

        dialog.window.set_text(title);
        *dialog.message.borrow_mut() = message.to_string();

        // Center on parent
        let (px, py) = parent.position();
        let (pw, ph) = parent.size();
        let (dw, dh) = dialog.window.size();
        dialog.window.set_position(
            px + (pw as i32 - dw as i32) / 2,
            py + (ph as i32 - dh as i32) / 2
        );

        nwg::dispatch_thread_events();

        let result = *dialog.result.borrow();
        result
    }

    fn on_init(&self) {
        self.message_label.set_text(&self.message.borrow());
        self.btn_no.set_focus();
    }

    fn on_yes(&self) {
        *self.result.borrow_mut() = true;
        self.close();
    }

    fn on_no(&self) {
        *self.result.borrow_mut() = false;
        self.close();
    }

    fn close(&self) {
        self.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }
}

// ============================================================================
// Message Dialog (simple info/warning/error)
// ============================================================================

/// Message dialog for showing information
#[derive(Default, nwd::NwgUi)]
pub struct MessageDialog {
    #[nwg_control(
        size: (400, 160),
        position: (400, 300),
        title: "Information",
        flags: "WINDOW|VISIBLE",
        topmost: true
    )]
    #[nwg_events(
        OnWindowClose: [MessageDialog::on_close],
        OnInit: [MessageDialog::on_init]
    )]
    pub window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 16)]
    message_font: nwg::Font,

    #[nwg_control(
        text: "",
        position: (15, 15),
        size: (370, 80),
        font: Some(&data.message_font)
    )]
    message_label: nwg::Label,

    #[nwg_control(text: "OK", position: (300, 105), size: (85, 35))]
    #[nwg_events(OnButtonClick: [MessageDialog::on_close])]
    btn_ok: nwg::Button,

    /// The message to display
    message: RefCell<String>,
}

impl MessageDialog {
    /// Show an information message
    #[allow(dead_code)]
    pub fn info(parent: &nwg::Window, title: &str, message: &str) {
        Self::show_message(parent, title, message);
    }

    fn show_message(parent: &nwg::Window, title: &str, message: &str) {
        let dialog = MessageDialog::build_ui(Default::default())
            .expect("Failed to build message dialog");

        dialog.window.set_text(title);
        *dialog.message.borrow_mut() = message.to_string();

        // Center on parent
        let (px, py) = parent.position();
        let (pw, ph) = parent.size();
        let (dw, dh) = dialog.window.size();
        dialog.window.set_position(
            px + (pw as i32 - dw as i32) / 2,
            py + (ph as i32 - dh as i32) / 2
        );

        nwg::dispatch_thread_events();
    }

    fn on_init(&self) {
        self.message_label.set_text(&self.message.borrow());
        self.btn_ok.set_focus();
    }

    fn on_close(&self) {
        self.window.set_visible(false);
        nwg::stop_thread_dispatch();
    }
}
