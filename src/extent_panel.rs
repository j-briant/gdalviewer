use egui::{RichText, Widget};
use gdal::vector::{Layer, LayerAccess};

#[derive(Default)]
pub struct Extent {
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
}

impl<'a> From<&Layer<'a>> for Extent {
    fn from(layer: &Layer<'a>) -> Self {
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

impl Widget for Extent {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.collapsing(RichText::new("Extent:").strong(), |ext| {
                ext.monospace("X Min: ");
                ext.label(self.xmin.to_string());
                ext.add_space(10.0);
                ext.monospace("X Max: ");
                ext.label(self.xmax.to_string());
                ext.add_space(10.0);
                ext.monospace("Y Min: ");
                ext.label(self.ymin.to_string());
                ext.add_space(10.0);
                ext.monospace("Y Max: ");
                ext.label(self.ymax.to_string());
                ext.add_space(10.0);
            });
        })
        .response
    }
}

impl Extent {
    pub fn new(layer: &Layer<'_>) -> Self {
        Self::from(layer)
    }
}
