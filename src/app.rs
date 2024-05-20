use eframe::egui;
use std::path::PathBuf;

pub struct ViewerPage {
    text: String,
    is_open: bool,
}

impl Default for ViewerPage {
    fn default() -> Self {
        Self {
            text: String::from("viewer"),
            is_open: false,
        }
    }
}

impl From<&PathBuf> for ViewerPage {
    fn from(value: &PathBuf) -> Self {
        match value.to_str() {
            Some(s) => Self {
                text: String::from(s),
                is_open: true,
            },
            None => Default::default(),
        }
    }
}

impl ViewerPage {
    fn new(s: String) -> Self {
        Self {
            text: s,
            is_open: true,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let viewport_id: egui::ViewportId = egui::ViewportId::from_hash_of("edit test");
        let viewport_builder = egui::ViewportBuilder::default().with_title(&self.text);
        let viewport_cb = |ctx: &egui::Context, _| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Name:");
                ui.label("Body:");
            });
        };
        ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_cb);
        if ctx.input(|i| i.viewport().close_requested()) {
            // Tell parent viewport that we should not show next frame:
            self.is_open = false;
        }
    }
}

#[derive(Default)]
pub struct ViewerApp {
    viewer: Vec<ViewerPage>,
}

impl ViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for ViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_files() {
                    self.viewer = path.iter().map(ViewerPage::from).collect();
                }
            }
        });
        /* if let Some(picked_path) = &self.picked_path {
            ui.horizontal(|ui| {
                ui.label("Picked file:");
                ui.monospace(picked_path);
                Window::new()
            });
        } */
    }
}

/* fn open_viewer(ctx: &egui::Context) {
    use egui::*;

    if ctx.input(|i| i.)
} */
