use egui::{
    emath,
    epaint::{self, Rect},
    pos2, Color32, Grid, Pos2, Sense, Stroke, Ui, Vec2
};

pub struct PaintEllipse {
    pub center: Pos2,
    pub radius: Vec2,
    pub fill: Color32,
    pub stroke: Stroke,
}

impl Default for PaintEllipse {
    fn default() -> Self {
        Self {
            center: pos2(150.0, 150.0),
            radius: Vec2{ x: 100.0, y: 50.0 },
            fill: Color32::from_rgb(100, 150, 200),
            stroke: Stroke::new(2.0, Color32::from_rgb(50, 100, 150)),
        }
    }
}

impl PaintEllipse {
    pub fn ui_control(&mut self, ui: &mut Ui) {
        ui.collapsing("Ellipse Properties", |ui| {
            Grid::new("ellipse_properties")
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
                    ui.add(egui::DragValue::new(&mut self.radius.x).speed(1.0));
                    ui.add(egui::DragValue::new(&mut self.radius.y).speed(1.0));
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

        let filled_ellipse = epaint::EllipseShape::filled(center_in_screen, self.radius, self.fill);
        painter.add(filled_ellipse);

        if self.stroke.width > 0.0 {
            let stroke_ellipse = epaint::EllipseShape::stroke(center_in_screen, self.radius, self.stroke);
            painter.add(stroke_ellipse);
        }

        response
    } 
}