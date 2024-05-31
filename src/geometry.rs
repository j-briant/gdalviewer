use egui::{RichText, Widget};
use gdal::vector::{Layer, LayerAccess};

#[derive(Default)]
pub struct GeometryInfo {
    geometry_name: Vec<String>,
    feature_number: usize,
}

impl Widget for GeometryInfo {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.collapsing(RichText::new("Geometry Info:").strong(), |ext| {
                ext.monospace("Geometry names: ");
                ext.label(self.geometry_name.join(", "));
                ext.add_space(10.0);
                ext.monospace("Number of features: ");
                ext.label(self.feature_number.to_string());
            });
            ui.add_space(10.0);
        })
        .response
    }
}

impl GeometryInfo {
    pub fn new(layer: &Layer<'_>) -> Self {
        Self {
            geometry_name: layer.defn().geom_fields().map(|f| f.name()).collect(),
            feature_number: layer.feature_count() as usize,
        }
    }
}
