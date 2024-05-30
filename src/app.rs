use crate::{
    data::dataset, extent_panel::Extent, features::FeatureWidget, fields::Fields,
    layer_panel::LayerList, srs::Srs,
};
use eframe::egui;
use egui::{Id, LayerId, Order, Painter};
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
        match &self.data {
            Some(d) => {
                let layer = &d.layer(self.active_layer).unwrap();
                let mut layer2 = d.layer(self.active_layer).unwrap();
                let feature = layer2.features().next().unwrap();

                egui::SidePanel::left("layer_list")
                    .show_separator_line(false)
                    .show(ctx, |ui| ui.add(LayerList::new(d, &mut self.active_layer)));

                egui::SidePanel::left("metadata").show(ctx, |ui| {
                    ui.add(Srs::new(layer));
                    ui.add(Extent::new(layer));
                    ui.add(Fields::new(layer));
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    //ui.add(GeometryPainter::new(&layer.feature(0).unwrap()));
                    ui.push_id("id_source", |ui| ui.add(FeatureWidget::new(&feature)));
                    //ui.label(feature.geometry().unwrap().wkt().unwrap())
                    //ui.add(GeometryPainter::new(&feature));
                });
            }
            None => {}
        }

        // Top-right corner closing button
        egui::Area::new(egui::Id::new("close_button_area"))
            .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(-10.0, 10.0))
            .show(ctx, |ui| {
                if ui.button("X").clicked() {
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
        ctx.set_pixels_per_point(1.2);
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
