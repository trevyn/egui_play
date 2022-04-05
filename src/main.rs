#![forbid(unsafe_code)]
#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
 let app = eframe_play::TemplateApp::default();
 let native_options = eframe::NativeOptions::default();
 eframe::run_native(Box::new(app), native_options);
}
