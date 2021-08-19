#![forbid(unsafe_code)]


pub mod task;
pub use task::Task;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = cortexa::App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

