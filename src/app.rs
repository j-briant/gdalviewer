use eframe::egui;
use egui::Align;
use egui_file_dialog::FileDialog;
use gdal::Dataset;
use std::path::PathBuf;

#[derive(Default)]
pub struct ViewerPage {
    text: String,
    is_open: bool,
}

impl ViewerPage {
    fn new(s: String) -> Self {
        Self {
            text: s,
            is_open: true,
        }
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
    }

    pub fn show(&mut self, ctx: &egui::Context, open: &bool) {
        let viewport_id: egui::ViewportId = egui::ViewportId::from_hash_of(format!("edit test"));
        let viewport_builder = egui::ViewportBuilder::default()
            .with_title(&self.text)
            .with_close_button(true);
        let viewport_cb = |ctx: &egui::Context, _| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Name:");
                ui.label("Body:");
            });
        };
        ctx.show_viewport_immediate(viewport_id, viewport_builder, viewport_cb);
    }
}
pub struct HomePage {
    picked_path: Option<String>,
    show_window: bool,
}

impl HomePage {
    fn new() -> Self {
        Self {
            picked_path: None,
            show_window: true,
        }
    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.picked_path = Some(path.display().to_string());
                }
            }
        });
    }
}

impl Default for HomePage {
    fn default() -> Self {
        HomePage {
            picked_path: None,
            show_window: true,
        }
    }
}

#[derive(Default)]
pub struct ViewerApp {
    homepage: HomePage,
    viewer: ViewerPage,
    datasets: Vec<Dataset>,
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
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.viewer = ViewerPage::new("text".into());
                }
            }
            self.viewer.show(ctx, &self.viewer.is_open);
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
