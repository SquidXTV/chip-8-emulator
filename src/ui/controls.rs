use eframe::emath::Vec2;
use eframe::epaint::{Color32, CornerRadius, Stroke, StrokeKind};
use egui::{vec2, Response, Sense, Ui, Widget};

pub const CONTROLS_WIDTH: f32 = 640.0;
pub const CONTROLS_HEIGHT: f32 = 50.0;

pub struct Controls {
    size: Vec2,
    fill: Color32,
    border: Color32,
}

impl Controls {

    pub fn new() -> Self {
        Self {
            size: vec2(CONTROLS_WIDTH, CONTROLS_HEIGHT),
            fill: Color32::BLACK,
            border: Color32::GRAY
        }
    }

}

impl Widget for Controls {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(self.size, Sense::empty());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            painter.rect_filled(rect, CornerRadius::same(8), self.fill);
            painter.rect_stroke(rect, CornerRadius::same(8), Stroke::new(2.0, self.border), StrokeKind::Outside);
        }

        response
    }
}
