use crate::data::dataset;
use eframe::egui;
use gdal::{
    vector::{Layer, LayerAccess},
    Dataset,
};
use std::path::PathBuf;

pub struct Metadata {
    layer_name: String,
    is_open: bool,
}

#[derive(Default)]
pub struct ViewerPage {
    data: Option<Dataset>,
    metadata: Option<Metadata>,
    is_open: bool,
}

/* impl Default for ViewerPage {
    fn default() -> Self {
        Self {
            data: None,
            is_open: false,
        }
    }
} */

impl From<&PathBuf> for ViewerPage {
    fn from(value: &PathBuf) -> Self {
        match dataset(PathBuf::from(value)) {
            Ok(d) => Self {
                metadata: Some(Metadata {
                    layer_name: d.layer(0).unwrap().name(),
                    is_open: true,
                }),
                data: Some(d),
                is_open: true,
            },
            Err(_) => ViewerPage::default(),
        }
    }
}

impl ViewerPage {
    pub fn show(&mut self, ctx: &egui::Context) {
        let viewport_id: egui::ViewportId = egui::ViewportId::from_hash_of("edit test");
        let viewport_builder = egui::ViewportBuilder::default().with_title("test");
        //let layer_names: Vec<String> = d.layers().map(|layer| layer.name()).collect();
        let viewport_cb = |ctx: &egui::Context, _| {
            egui::SidePanel::left("layers").show(ctx, |ui| {
                ui.label("Layers:");
                for l in self.data.as_ref().unwrap().layers() {
                    if ui.button(l.name()).clicked() {
                        self.metadata = Some(Metadata {
                            layer_name: l.name(),
                            is_open: true,
                        })
                    }
                }
            });
            egui::SidePanel::left("metadata").show(ctx, |ui| {
                ui.label(
                    self.data
                        .as_ref()
                        .unwrap()
                        .layer_by_name(&self.metadata.as_ref().unwrap().layer_name)
                        .unwrap()
                        .feature_count()
                        .to_string(),
                )
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
    viewer: ViewerPage,
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
                    self.viewer = ViewerPage::try_from(&path).unwrap();
                }
            }
            if self.viewer.is_open {
                self.viewer.show(ctx);
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
