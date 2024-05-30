use egui::{RichText, Widget};
use gdal::vector::LayerAccess;
use gdal::Dataset;

pub struct LayerList<'a> {
    layer_list: Vec<String>,
    pub active_layer: &'a mut isize,
}

/* impl<'a> From<&Dataset> for LayerList<'a> {
    fn from(dataset: &Dataset) -> Self {
        let ll: Vec<String> = dataset.layers().map(|l| l.name()).collect();
        Self {
            layer_list: ll,
            active_layer: &mut 0,
        }
    }
} */

impl<'a> Widget for LayerList<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.monospace(RichText::new("Layer list:").strong());
            ui.add_space(10.0);
            self.layer_list.iter().enumerate().for_each(|(i, l)| {
                if ui.button(l).clicked() {
                    *self.active_layer = i as isize
                }
            })
        })
        .response
    }
}

impl<'a> LayerList<'a> {
    /*     pub fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("layer_list").show(ctx, |ui| {
            ui.label("Layer list:");
            ui.add_space(10.0);
            self.layer_list.iter().enumerate().for_each(|(i, l)| {
                if ui.button(l).clicked() {
                    *self.active_layer = i as isize
                }
            })
        });
    } */

    pub fn new(dataset: &Dataset, active_layer: &'a mut isize) -> Self {
        let ll: Vec<String> = dataset.layers().map(|l| l.name()).collect();
        Self {
            layer_list: ll,
            active_layer,
        }
    }
}
