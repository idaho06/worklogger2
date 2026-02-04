// Hides console window in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod file_ops;
mod dialogs;
mod app;

use app::WorkLoggerApp;
use native_windows_gui as nwg;

fn main() {
    // Initialize NWG
    nwg::init().expect("Failed to initialize Native Windows GUI");

    // Set global font
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set font");

    // Build and run the application
    let _app = WorkLoggerApp::build()
        .expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
