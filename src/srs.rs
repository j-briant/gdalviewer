use egui::{RichText, Widget};
use gdal::vector::{Layer, LayerAccess};

#[derive(Default)]
pub struct Srs {
    wkt: String,
}

impl From<&Layer<'_>> for Srs {
    fn from(layer: &Layer<'_>) -> Self {
        if let Some(sr) = layer.spatial_ref() {
            Self {
                wkt: sr.to_pretty_wkt().unwrap_or(String::from("")),
            }
        } else {
            Self::default()
        }
    }
}

impl Widget for Srs {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.collapsing(RichText::new("SRS:").strong(), |ext| {
                ext.monospace(RichText::new("WKT: "));
                ext.label(self.wkt);
                ext.add_space(10.0);
            });
        })
        .response
    }
}

impl Srs {
    pub fn new(layer: &Layer<'_>) -> Self {
        Self::from(layer)
    }
}
