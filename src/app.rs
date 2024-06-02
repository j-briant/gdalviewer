use crate::{
    data::dataset,
    extent::Extent,
    fields::Fields,
    geometry::GeometryInfo,
    layer_panel::LayerList,
    map::{Map, MapState},
    srs::Srs,
};
use eframe::{egui, Frame};
use gdal::Dataset;
use std::path::PathBuf;

#[derive(Default)]
pub struct ViewerPage {
    map_state: MapState,
    driver: String,
    active_layer: isize,
    data: Option<Dataset>,
    is_open: bool,
}

impl From<&PathBuf> for ViewerPage {
    fn from(value: &PathBuf) -> Self {
        match dataset(PathBuf::from(value)) {
            Ok(d) => Self {
                map_state: MapState::new(),
                driver: d.driver().long_name(),
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
                //let feature = layer2.features().last().unwrap();

                egui::SidePanel::left("dataset_info")
                    .show_separator_line(true)
                    .show(ctx, |ui| {
                        ui.heading("Dataset information");
                        ui.add_space(10.0);
                        ui.monospace(egui::RichText::new("Driver:").strong());
                        ui.label(&self.driver);
                        ui.add_space(10.0);
                        ui.add(LayerList::new(d, &mut self.active_layer));
                    });

                egui::SidePanel::left("layer_info").show(ctx, |ui| {
                    ui.heading("Layer information");
                    ui.add_space(10.0);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                            ui.add(Srs::new(layer));
                            ui.add(Extent::new(layer));
                            ui.add(Fields::new(layer));
                            ui.add(GeometryInfo::new(layer));
                        })
                    });
                });

                egui::CentralPanel::default().show(ctx, |ui| {
                    //ui.push_id("id_source", |ui| ui.add(FeatureWidget::new(&feature)));
                    let mut map = Map::new(&mut self.map_state, &mut layer2);
                    map.update(ui, ctx);
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
