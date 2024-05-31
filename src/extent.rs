use egui::{RichText, Widget};
use gdal::vector::{Layer, LayerAccess};

#[derive(Default)]
pub struct Extent {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

impl Widget for Extent {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.collapsing(RichText::new("Extent:").strong(), |ext| {
                ext.with_layout(egui::Layout::left_to_right(egui::Align::Min), |lr| {
                    lr.monospace("X Min: ");
                    lr.label(self.xmin.to_string());
                });
                ext.with_layout(egui::Layout::left_to_right(egui::Align::Min), |lr| {
                    lr.monospace("X Max: ");
                    lr.label(self.xmax.to_string());
                });
                ext.with_layout(egui::Layout::left_to_right(egui::Align::Min), |lr| {
                    lr.monospace("Y Min: ");
                    lr.label(self.ymin.to_string());
                });
                ext.with_layout(egui::Layout::left_to_right(egui::Align::Min), |lr| {
                    lr.monospace("Y Max: ");
                    lr.label(self.ymax.to_string());
                });
            });
            ui.add_space(10.0);
        })
        .response
    }
}

impl Extent {
    pub fn new(layer: &Layer<'_>) -> Self {
        if let Ok(l) = layer.get_extent() {
            Self {
                xmin: l.MinX,
                xmax: l.MaxX,
                ymin: l.MinY,
                ymax: l.MaxY,
            }
        } else {
            Extent::default()
        }
    }
}
