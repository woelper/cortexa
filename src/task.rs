use chrono::NaiveDateTime;
use std::{cmp::Ordering, time::Duration};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Cadence {
    Weekly(u8),
    Monthly(u8),
    Daily(u8),
    Custom(Duration)
}


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum Deadline {
    /// No deadline
    None,
    /// Deadline is a single date
    Date(NaiveDateTime),
    /// Deadline is periodic with a start and a cadence
    Period(NaiveDateTime, Cadence),
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubTask {
    pub done: bool,
    pub description: String,
}

impl PartialOrd for SubTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.done.partial_cmp(&other.done)
    }
}

impl Ord for SubTask {
    fn cmp(&self, other: &Self) -> Ordering {
        self.done.cmp(&other.done)

    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone)]
pub enum NoteContent {
    Text(String),
    Subtasks(Vec<SubTask>),
}

impl NoteContent {
    pub fn to_text(&self) -> NoteContent {
        match self {
            NoteContent::Text(t) => NoteContent::Text(t.clone()),
            NoteContent::Subtasks(st) => NoteContent::Text(
                st.iter()
                    .map(|t| format!("{}{}", if t.done { "x " } else { "" }, t.description))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        }
    }
    pub fn to_subtasks(&self) -> NoteContent {
        match self {
            NoteContent::Text(t) => NoteContent::Subtasks(
                t.lines()
                    .map(|l| SubTask {
                        done: l.to_lowercase().starts_with("x"),
                        description: l.replace("x ", ""),
                    })
                    .collect(),
            ),
            NoteContent::Subtasks(st) => NoteContent::Subtasks(st.clone()),
        }
    }
}

impl Default for NoteContent {
    fn default() -> Self {
        Self::Text("".into())
    }
}

impl Default for Deadline {
    fn default() -> Self {
        Deadline::None
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Task {
    pub name: String,
    pub description: NoteContent,
    pub deadline: Deadline,
    pub priority: f32,
}
