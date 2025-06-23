use egui::{
    emath,
    epaint::{self, Rect},
    pos2, Color32, Grid, Pos2, Sense, Stroke, Ui, Vec2
};
pub struct PaintCircle {
    pub center: Pos2,
    pub radius: f32,
    pub fill: Color32,
    pub stroke: Stroke,
}

impl Default for PaintCircle {
    fn default() -> Self {
        Self {
            center: pos2(150.0, 150.0),
            radius: 100.0,
            fill: Color32::from_rgb(100, 150, 200),
            stroke: Stroke::new(2.0, Color32::from_rgb(50, 100, 150)),
        }
    }
}

impl PaintCircle {
    pub fn ui_control(&mut self, ui: &mut Ui) {
        ui.collapsing("Circle Properties", |ui| {
            Grid::new("circle_properties")
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

                    ui.label("Radius");
                    ui.add(egui::DragValue::new(&mut self.radius).speed(1.0));
                    ui.end_row();

                    ui.label("Center");
                    ui.add(egui::DragValue::new(&mut self.center.x).speed(1.0));
                    ui.add(egui::DragValue::new(&mut self.center.y).speed(1.0));
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

        let center_in_screen = to_screen.transform_pos(self.center);

        let filled_circle = epaint::CircleShape::filled(center_in_screen, self.radius, self.fill);
        painter.add(filled_circle);

        if self.stroke.width > 0.0 {
            let stroke_circle = epaint::CircleShape::stroke(center_in_screen, self.radius, self.stroke);
            painter.add(stroke_circle);
        }

        response
    } 
}