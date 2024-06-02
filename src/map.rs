use egui::{Context, Painter, Pos2, Ui, Vec2};
use gdal::vector::{Layer, LayerAccess};

#[derive(Default)]
pub struct MapState {
    offset: Vec2,
    zoom: f32,
}

impl MapState {
    pub fn new() -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
            zoom: 1.0,
        }
    }

    pub fn handle_drag(&mut self, ctx: &Context) {
        if ctx.input(|i| i.pointer.primary_down()) {
            self.offset += ctx.input(|i| i.pointer.delta());
        }
    }

    pub fn handle_zoom(&mut self, ctx: &Context) {
        let zoom_delta = ctx.input(|i| i.smooth_scroll_delta.y / (1000.0));
        self.zoom += zoom_delta;
    }
}

pub struct Map<'a> {
    map_state: &'a mut MapState,
    layer: &'a mut Layer<'a>,
}

impl<'a> Map<'a> {
    pub fn new(map_state: &'a mut MapState, layer: &'a mut Layer<'a>) -> Self {
        Self { map_state, layer }
    }

    pub fn update(&mut self, ui: &mut Ui, ctx: &Context) {
        self.map_state.handle_drag(ctx);
        self.map_state.handle_zoom(ctx);
        self.render_map(ui);
    }

    pub fn render_map(&mut self, ui: &mut Ui) -> egui::Response {
        ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Min),
            |inner| {
                let available_size = inner.available_size();
                let (_, painter) =
                    inner.allocate_painter(inner.available_size(), egui::Sense::hover());

                let rect = painter.clip_rect();
                painter.rect_stroke(rect, 0.0, egui::Stroke::new(2.0, egui::Color32::RED));

                self.render_layer(&painter, available_size);
            },
        )
        .response
    }

    fn render_layer(&mut self, painter: &Painter, available_size: Vec2) {
        let rect = painter.clip_rect();
        let bbox = self.layer.get_extent().unwrap();
        for feature in self.layer.features() {
            if let Some(geometry) = feature.geometry() {
                let mut points: Vec<(f64, f64, f64)> = vec![];
                if !geometry.geometry_name().contains("POLYGON") {
                    points.extend(geometry.get_point_vec());
                } else {
                    for idx in 0..geometry.geometry_count() {
                        points.extend(geometry.get_geometry(idx).get_point_vec());
                    }
                }

                // Transform geo referenced coordinates into screen coordinates
                let transform = |x: f64, y: f64| {
                    let mut ratio_x = 0.0;
                    let mut ratio_y = 0.0;
                    if bbox.MaxX != bbox.MinX && bbox.MaxY != bbox.MinY {
                        ratio_x += bbox.MaxX - bbox.MinX;
                        ratio_y += bbox.MaxY - bbox.MinY;
                    } else {
                        ratio_x += bbox.MaxX;
                        ratio_y += bbox.MaxY;
                    }

                    let x_transformed = rect.min.x as f64
                        + self.map_state.zoom as f64
                            * available_size.x as f64
                            * (x / ratio_x - (bbox.MaxX + bbox.MinX) / (2.0 * ratio_x) + 0.5)
                        + self.map_state.offset.x as f64;

                    let y_transformed = rect.min.y as f64
                        + self.map_state.zoom as f64
                            * available_size.y as f64
                            * (-y / ratio_y + (bbox.MaxY + bbox.MinY) / (2.0 * ratio_y) + 0.5)
                        + self.map_state.offset.y as f64;
                    (x_transformed, y_transformed)
                };

                let coord: Vec<Pos2> = points
                    .iter()
                    .map(|(x, y, _)| {
                        let (x, y) = transform(*x, *y);
                        Pos2::new(x as f32, y as f32)
                    })
                    .collect();

                coord.iter().for_each(|c| {
                    painter.add(egui::Shape::circle_stroke(
                        *c,
                        5.0,
                        egui::Stroke::new(2.0, egui::Color32::WHITE),
                    ));
                });
                painter.add(egui::Shape::line(
                    coord,
                    egui::Stroke::new(1.0, egui::Color32::RED),
                ));
            }
        }
    }
}
