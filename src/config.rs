use winreg::enums::*;
use winreg::RegKey;
use std::path::PathBuf;

/// Registry path for storing configuration
const REG_PATH: &str = r"Software\AkinoSoft\WorkLogger";

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    /// GUI theme: "dark" or "light"
    pub gui_theme: String,

    /// Project name prefix for log files (e.g., "WorkLog")
    pub project_name: String,

    /// Directory where log files are saved
    pub log_path: PathBuf,

    /// Whether to wrap text in the editor
    pub word_wrap: bool,

    /// Whether to insert current time when app starts
    pub insert_time_at_start: bool,
}

impl Default for Config {
    fn default() -> Self {
        // Default log path is user's Documents folder
        let documents = dirs_next::document_dir()
            .unwrap_or_else(|| PathBuf::from("C:\\"));

        Config {
            gui_theme: "dark".to_string(),
            project_name: "WorkLog".to_string(),
            log_path: documents,
            word_wrap: false,
            insert_time_at_start: true,
        }
    }
}

impl Config {
    /// Load configuration from Windows Registry
    /// Returns default config if registry keys don't exist
    pub fn load() -> Self {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        match hkcu.open_subkey(REG_PATH) {
            Ok(key) => {
                let default = Config::default();

                Config {
                    gui_theme: key.get_value("gui_theme")
                        .unwrap_or(default.gui_theme),

                    project_name: key.get_value("project_name")
                        .unwrap_or(default.project_name),

                    log_path: key.get_value::<String, _>("log_path")
                        .map(PathBuf::from)
                        .unwrap_or(default.log_path),

                    word_wrap: key.get_value::<u32, _>("word_wrap")
                        .map(|v| v != 0)
                        .unwrap_or(default.word_wrap),

                    insert_time_at_start: key.get_value::<u32, _>("insert_time_at_start")
                        .map(|v| v != 0)
                        .unwrap_or(default.insert_time_at_start),
                }
            }
            Err(_) => {
                // Registry key doesn't exist, return defaults
                // and create the key with defaults
                let config = Config::default();
                config.save();
                config
            }
        }
    }

    /// Save current configuration to Windows Registry
    pub fn save(&self) {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);

        // Create key if it doesn't exist
        let (key, _) = hkcu.create_subkey(REG_PATH)
            .expect("Failed to create registry key");

        key.set_value("gui_theme", &self.gui_theme).ok();
        key.set_value("project_name", &self.project_name).ok();
        key.set_value("log_path", &self.log_path.to_string_lossy().to_string()).ok();
        key.set_value("word_wrap", &(self.word_wrap as u32)).ok();
        key.set_value("insert_time_at_start", &(self.insert_time_at_start as u32)).ok();
    }

    /// Toggle theme between dark and light
    pub fn toggle_theme(&mut self) {
        self.gui_theme = if self.gui_theme == "dark" {
            "light".to_string()
        } else {
            "dark".to_string()
        };
        self.save();
    }

    /// Update project name and save
    pub fn set_project_name(&mut self, name: &str) {
        // Sanitize: replace spaces with underscores, limit length
        let sanitized: String = name.chars()
            .take(80)
            .map(|c| if c == ' ' { '_' } else { c })
            .collect();
        self.project_name = sanitized;
        self.save();
    }

    /// Update log path and save
    pub fn set_log_path(&mut self, path: PathBuf) {
        self.log_path = path;
        self.save();
    }

    /// Toggle word wrap and save
    pub fn toggle_word_wrap(&mut self) {
        self.word_wrap = !self.word_wrap;
        self.save();
    }

    /// Toggle insert time at start and save
    pub fn toggle_insert_time(&mut self) {
        self.insert_time_at_start = !self.insert_time_at_start;
        self.save();
    }

    /// Check if dark theme is active
    pub fn is_dark_theme(&self) -> bool {
        self.gui_theme == "dark"
    }
}
