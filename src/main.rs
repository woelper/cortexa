#![forbid(unsafe_code)]

use chrono::{NaiveDateTime};
use std::time::Duration;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app = cortexa::App::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub enum Deadline {
    None,
    Date(NaiveDateTime),
    Period(Duration)
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
pub struct Task {
    pub name: String,
    pub deadline: Deadline
}