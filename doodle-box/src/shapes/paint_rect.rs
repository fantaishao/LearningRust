use egui::{
    emath,
    epaint::{self, Rect, Brush},
    pos2, Color32, Grid, Pos2, Sense, Stroke, StrokeKind, Ui, Vec2, CornerRadius
};
use std::sync::Arc;

pub struct PaintRect {
    pub rect: Rect,
    pub corner_radius: CornerRadius,
    pub fill: Color32,
    pub stroke: Stroke,
    pub stroke_kind: StrokeKind,
    pub round_to_pixels: Option<bool>,
    pub blur_width: f32,
    pub brush: Option<Arc<Brush>>,
}

impl Default for PaintRect {
    fn default() -> Self {
        Self {
            rect: Rect::from_min_size(pos2(50.0, 50.0), Vec2::new(200.0, 150.0)),
            corner_radius: CornerRadius::default(),
            fill: Color32::from_rgb(100, 150, 200),
            stroke: Stroke::new(2.0, Color32::from_rgb(50, 100, 150)),
            stroke_kind: StrokeKind::Outside,
            round_to_pixels: None,
            blur_width: 0.0,
            brush: Default::default(),
        }
    }
}

impl PaintRect {
    pub fn ui_control(&mut self, ui: &mut Ui) {
        ui.collapsing("Rectangle Properties", |ui| {
            Grid::new("rect_properties")
                .num_columns(2)
                .spacing([12.0, 8.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Fill color");
                    ui.color_edit_button_srgba(&mut self.fill);
                    ui.end_row();

                    ui.label("Stroke");
                    ui.add(&mut self.stroke);
                    ui.end_row();

                    ui.label("Corner radius");
                    ui.add(egui::DragValue::new(&mut self.corner_radius.nw).speed(1.0));
                    ui.end_row();

                    ui.label("Blur width");
                    ui.add(egui::DragValue::new(&mut self.blur_width).speed(0.1));
                    ui.end_row();
                });
        });
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (response, painter) =
            ui.allocate_painter(Vec2::new(ui.available_width(), 300.0), Sense::hover());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.size()),
            response.rect,
        );

        // Transform the rectangle to screen coordinates
        let rect_in_screen = to_screen.transform_rect(self.rect);

        // Create filled rectangle
        let filled_rect = epaint::RectShape::filled(
            rect_in_screen,
            self.corner_radius,
            self.fill,
        );
        painter.add(filled_rect);

        // Add stroke if needed
        if self.stroke.width > 0.0 {
            let stroke_rect = epaint::RectShape::stroke(
                rect_in_screen,
                self.corner_radius,
                self.stroke,
                self.stroke_kind,
            );
            painter.add(stroke_rect);
        }

        response
    }
}
