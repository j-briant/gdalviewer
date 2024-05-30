use egui::{Color32, Id, LayerId, Painter, Pos2, Shape, Stroke, Ui, Widget};
use gdal::vector::{Feature, Geometry};

pub struct FeatureWidget<'a> {
    pub geometry: &'a Geometry,
}

impl<'a> Widget for FeatureWidget<'a> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.allocate_ui_with_layout(ui.available_size(), egui::Layout::default(), |ui| {
            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                self.paint_background(painter, rect);
                self.paint_geometry(painter, rect);
            }
        })
        .response
    }
}

impl<'a> FeatureWidget<'a> {
    pub fn new(feature: &'a Feature<'a>) -> Self {
        let geometry = feature.geometry().unwrap();
        Self { geometry }
    }

    pub fn ui(self, ui: &mut Ui) {
        ui.allocate_ui_with_layout(ui.available_size(), egui::Layout::default(), |ui| {
            let (rect, response) =
                ui.allocate_exact_size(ui.available_size(), egui::Sense::hover());
            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                self.paint_background(painter, rect);
                self.paint_geometry(painter, rect);
            }
            response
        });
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
                scale_y += height / (bbox.MaxY - bbox.MinY);
            }
            let x_transformed = rect.min.x as f64 + width / 2.0 + (x - bbox.MinX) * scale_x;
            let y_transformed = rect.min.y as f64 + height / 2.0 + (y - bbox.MaxY) * scale_y; // y-axis is inverted
            (x_transformed, y_transformed)
        };

        for idx in 0..=self.geometry.geometry_count() {
            let points: Vec<(f64, f64, f64)> = self.geometry.get_geometry(idx).get_point_vec();
            let coord: Vec<Pos2> = points
                .iter()
                .map(|&(x, y, _)| {
                    let (x, y) = transform(x, y);
                    Pos2::new(x as f32, y as f32)
                })
                .collect();
            for p in coord {
                painter.add(Shape::circle_stroke(
                    p,
                    10.0,
                    egui::Stroke::new(2.0, Color32::WHITE),
                ));
            }
        }
    }
}

/*
#[derive(Default)]
pub struct GeometryPainter {
    point_list: Vec<Pos2>,
    marker_radius: f32,
}

impl From<&Feature<'_>> for GeometryPainter {
    fn from(feature: &Feature<'_>) -> Self {
        if let Some(g) = feature.geometry() {
            let coord = g.get_point_vec();
            let pos2: Vec<Pos2> = coord
                .iter()
                .map(|(x, y, _)| {
                    Pos2::from((*x as f32 - *x as f32 + 100.0, *y as f32 - *y as f32 + 100.0))
                })
                .collect();
            Self {
                point_list: pos2,
                marker_radius: 10.0,
                ..Default::default()
            }
        } else {
            Self::default()
        }
    }
}

impl Widget for GeometryPainter {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
        ui.vertical(|ui| {
            ui.painter()
                .with_clip_rect(rect)
                .add(egui::Shape::circle_filled(
                    self.point_list[0],
                    self.marker_radius,
                    Color32::from_rgb(255, 0, 0),
                ))
        })
        .response
    }
}

impl GeometryPainter {
    pub fn new(feature: &Feature<'_>) -> Self {
        Self::from(feature)
    }
} */
