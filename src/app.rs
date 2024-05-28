use crate::data::dataset;
use eframe::egui;
use gdal::{vector::LayerAccess, Dataset};
use std::path::PathBuf;

/* pub struct Metadata {
    layer_name: String,
    is_open: bool,
} */

#[derive(Default)]
pub struct ViewerPage {
    active_layer: isize,
    data: Option<Dataset>,
    is_open: bool,
}

/* impl Default for ViewerPage {
    fn default() -> Self {
        Self {
            active_layer: 0,
            data: None,
            is_open: false,
        }
    }
} */

impl From<&PathBuf> for ViewerPage {
    fn from(value: &PathBuf) -> Self {
        match dataset(PathBuf::from(value)) {
            Ok(d) => Self {
                active_layer: 0,
                data: Some(d),
                is_open: true,
            },
            Err(_) => ViewerPage::default(),
        }
    }
}

impl ViewerPage {
    pub fn show(&mut self, ctx: &egui::Context) {
        //egui::Window::new("Viewer").title_bar(false).show(ctx, |_| {
        egui::SidePanel::left("layers").show(ctx, |ui| {
            ui.label("Layers:");
            ui.add_space(5.0);
            self.data
                .as_ref()
                .unwrap()
                .layers()
                .enumerate()
                .for_each(|(i, l)| {
                    if ui.button(l.name()).clicked() {
                        self.active_layer = i as isize;
                    }
                });
        });
        egui::SidePanel::left("metadata").show(ctx, |ui| {
            ui.label("Feature count:");
            ui.label(
                self.data
                    .as_ref()
                    .unwrap()
                    .layer(self.active_layer)
                    .unwrap()
                    .feature_count()
                    .to_string(),
            );
            ui.label("X max:");
            ui.label(
                self.data
                    .as_ref()
                    .unwrap()
                    .layer(self.active_layer)
                    .unwrap()
                    .get_extent()
                    .unwrap()
                    .MaxX
                    .to_string(),
            );
            ui.label("X min:");
            ui.label(
                self.data
                    .as_ref()
                    .unwrap()
                    .layer(self.active_layer)
                    .unwrap()
                    .get_extent()
                    .unwrap()
                    .MinX
                    .to_string(),
            );
            ui.label("Y max:");
            ui.label(
                self.data
                    .as_ref()
                    .unwrap()
                    .layer(self.active_layer)
                    .unwrap()
                    .get_extent()
                    .unwrap()
                    .MaxY
                    .to_string(),
            );
            ui.label("Y min:");
            ui.label(
                self.data
                    .as_ref()
                    .unwrap()
                    .layer(self.active_layer)
                    .unwrap()
                    .get_extent()
                    .unwrap()
                    .MinY
                    .to_string(),
            );
            ui.label("Spatial reference:");
            ui.label(
                self.data
                    .as_ref()
                    .unwrap()
                    .layer(self.active_layer)
                    .unwrap()
                    .spatial_ref()
                    .unwrap()
                    .to_pretty_wkt()
                    .unwrap(),
            );
        });
        egui::SidePanel::right("closing").show(ctx, |ui| {
            if ui.button("x").clicked() {
                self.is_open = false;
            }
        });
        //});
    }
}

#[derive(Default)]
pub struct ViewerApp {
    viewer: ViewerPage,
}

impl ViewerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for ViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");
            if ui.button("Open file…").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.viewer = ViewerPage::from(&path);
                }
            }

            if self.viewer.is_open {
                self.viewer.show(ctx);
            }
        });
    }
}

/* fn open_viewer(ctx: &egui::Context) {
    use egui::*;

    if ctx.input(|i| i.)
} */
