#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

use eframe::{egui};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    
    let app = lst_gui::LstApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2::new(300.0, 225.0));
    eframe::run_native(Box::new(app), native_options);
}
