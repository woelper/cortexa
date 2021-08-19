use chrono::{NaiveDateTime};
use std::time::Duration;


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum Deadline {
    None,
    Date(NaiveDateTime),
    Period(Duration)
}

impl Default for Deadline {
    fn default() -> Self {
        Deadline::None
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Default)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub deadline: Deadline
}