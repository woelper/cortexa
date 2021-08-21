use crate::task::{
    NoteContent::{Subtasks, Text},
    SubTask,
};
use eframe::{
    egui::{self, DragValue, Ui},
    epi,
};

// use super::task;
use super::task::Task;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct App {
    tasks: Vec<Task>,
    // this how you opt-out of serialization of a member
    // #[cfg_attr(feature = "persistence", serde(skip))]
}

impl Default for App {
    fn default() -> Self {
        Self { tasks: vec![] }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        "egui template"
    }

    /// Called by the framework to load old app state (if any).
    #[cfg(feature = "persistence")]
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        storage: Option<&dyn epi::Storage>,
    ) {
        if let Some(storage) = storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        ctx.set_visuals(egui::Visuals::light());
    }

    /// Called by the frame work to save state before shutdown.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { tasks } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            if ui.button("new task").clicked() {
                tasks.push(Task::default());
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("egui template");

            for task in tasks {
                draw_task(task, ui);
            }
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}

fn draw_task(task: &mut Task, ui: &mut Ui) {
    // let l = egui::Layout::default().with_main_wrap(true);
    ui.group(|ui| {
        ui.text_edit_singleline(&mut task.name);
        ui.add(DragValue::new(&mut task.priority));
        match &mut task.description {
            Text(t) => {
                ui.text_edit_multiline(t);
                if ui.button("-> tasks").clicked() {
                    task.description = task.description.to_subtasks();
                }
            }
            Subtasks(st) => {
                for t in st {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut t.done, "");
                        ui.text_edit_singleline(&mut t.description);
                    });
                }
                if ui.button("-> text").clicked() {
                    task.description = task.description.to_text( )
                }
            }
        }
    });
}
