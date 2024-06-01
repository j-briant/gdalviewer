use egui::{debug_text::print, Context, Painter, Pos2, Ui, Vec2};
use gdal::vector::{Layer, LayerAccess};

#[derive(Default)]
pub struct MapState {
    offset: Vec2,
    last_mouse_pos: Option<Pos2>,
    zoom: f32,
}

impl MapState {
    pub fn new() -> Self {
        Self {
            offset: Vec2::new(0.0, 0.0),
            last_mouse_pos: Some(Pos2::new(0.0, 0.0)),
            zoom: 1.0,
        }
    }

    pub fn handle_input(&mut self, ctx: &Context) {
        if ctx.input(|i| i.pointer.primary_down()) {
            self.offset += ctx.input(|i| i.pointer.delta());
        }
    }

    pub fn handle_zoom(&mut self, ctx: &Context) {
        let zoom_delta = ctx.input(|i| i.smooth_scroll_delta.y / (100.0));
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
        self.map_state.handle_input(ctx);
        self.map_state.handle_zoom(ctx);
        println!("{}", self.map_state.zoom);
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

                for point in points {
                    /* let x = (point.0 as f32 + 10.0 - point.0 as f32)
                    + self.map_state.zoom
                    + self.map_state.offset.x
                    + available_size.x / 2.0; */
                    /* let y = (point.1 as f32 + 10.0 - point.1 as f32)
                    + self.map_state.zoom
                    + self.map_state.offset.y
                    + available_size.y / 2.0; */

                    let x = point.0 as f32 * available_size.x * self.map_state.zoom
                        / (geometry.envelope().MaxX) as f32
                        + self.map_state.offset.x;

                    let y = -(point.1 as f32 * available_size.y * self.map_state.zoom)
                        / (geometry.envelope().MaxY) as f32
                        + available_size.y
                        + self.map_state.offset.y;

                    println!("x: {}\ny: {}", x, y);
                    println!("zoom: {}", self.map_state.zoom);
                    println!("mouse pos: {:?}", self.map_state.last_mouse_pos);
                    println!("offset: {:?}", self.map_state.offset);

                    painter.circle_filled(Pos2::new(x, y), 5.0, egui::Color32::WHITE);
                }
            }
        }
    }
}
