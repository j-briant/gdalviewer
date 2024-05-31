use egui::{Color32, Painter, Pos2, Shape, Ui, Widget};
use gdal::vector::{Feature, Geometry};

pub struct FeatureWidget<'a> {
    pub geometry: &'a Geometry,
}

impl<'a> Widget for FeatureWidget<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                let (_, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                let rect = painter.clip_rect();
                if ui.is_rect_visible(rect) {
                    let painter = ui.painter();
                    self.paint_background(painter, rect);
                    self.paint_geometry(painter, rect);
                }
            },
        )
        .response
    }
}

impl<'a> FeatureWidget<'a> {
    pub fn new(feature: &'a Feature<'a>) -> Self {
        let geometry = feature.geometry().unwrap();
        Self { geometry }
    }

    fn paint_background(&self, painter: &Painter, rect: egui::Rect) {
        painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, Color32::RED));
        //painter.rect_filled(rect, 0.0, Color32::from_rgb(200, 200, 200)); // Light gray background
    }

    fn paint_geometry(&self, painter: &Painter, rect: egui::Rect) {
        let bbox = self.geometry.envelope();
        let transform = |x: f64, y: f64| {
            let width = rect.width() as f64;
            let height = rect.height() as f64;
            let mut scale_x: f64 = 0.0;
            let mut scale_y: f64 = 0.0;
            if (bbox.MaxX - bbox.MinX) != 0.0 && (bbox.MaxY - bbox.MinY) != 0.0 {
                scale_x += width / (bbox.MaxX - bbox.MinX);
                scale_y += -height / (bbox.MaxY - bbox.MinY);
            }
            let x_transformed = (rect.min.x) as f64 + (x - bbox.MinX) * scale_x;
            let y_transformed = (rect.min.y) as f64 + (y - bbox.MaxY) * scale_y; // y-axis is inverted
            (x_transformed, y_transformed)
        };

        let mut points: Vec<(f64, f64, f64)> = vec![];
        if !self.geometry.geometry_name().contains("POLYGON") {
            points.extend(self.geometry.get_point_vec());
        } else {
            for idx in 0..self.geometry.geometry_count() {
                points.extend(self.geometry.get_geometry(idx).get_point_vec());
            }
        }
        let coord: Vec<Pos2> = points
            .iter()
            .map(|&(x, y, _)| {
                let (x, y) = transform(x, y);
                Pos2::new(x as f32, y as f32)
            })
            .collect();
        coord.iter().for_each(|c| {
            painter.add(Shape::circle_stroke(
                *c,
                5.0,
                egui::Stroke::new(2.0, Color32::WHITE),
            ));
        });
        painter.add(Shape::line(coord, egui::Stroke::new(1.0, Color32::RED)));
    }
}
