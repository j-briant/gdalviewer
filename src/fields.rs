use egui::{RichText, Widget};
use egui_extras::{Column, TableBuilder};
use gdal::vector::{Layer, LayerAccess};

#[derive(Default)]
pub struct Fields {
    geom_field: Vec<String>,
    fields: Vec<(String, u32)>,
}

impl Widget for Fields {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.collapsing(RichText::new("Fields:").strong(), |ui| {
            TableBuilder::new(ui)
                .resizable(false)
                .striped(true)
                .column(Column::auto().resizable(true))
                .column(Column::remainder())
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.monospace("Field name");
                    });
                    header.col(|ui| {
                        ui.monospace("Field type");
                    });
                })
                .body(|body| {
                    body.rows(15.0, self.fields.len(), |mut row| {
                        let rindex = row.index();
                        row.col(|ui| {
                            ui.label(&self.fields[rindex].0);
                        });
                        row.col(|ui| {
                            ui.label(&self.fields[rindex].1.to_string());
                        });
                    });
                });
            ui.add_space(10.0);
        })
        .header_response
    }
}

impl Fields {
    pub fn new(layer: &Layer<'_>) -> Self {
        let fields: Vec<(String, u32)> = layer
            .defn()
            .fields()
            .map(|f| (f.name(), f.field_type()))
            .collect();
        let geom_field: Vec<String> = layer.defn().geom_fields().map(|g| g.name()).collect();
        Self { geom_field, fields }
    }
}
