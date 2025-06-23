use eframe::egui;

use doodle_box::shapes::paint_bezier::PaintBezier;
use doodle_box::shapes::paint_rect::PaintRect;
use doodle_box::shapes::paint_circle::PaintCircle;
use doodle_box::shapes::paint_ellipse::PaintEllipse;
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("DOODLE BOX", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Default)]
struct MyEguiApp {
    paint_bezier: PaintBezier,
    paint_rect: PaintRect,
    paint_circle: PaintCircle,
    paint_ellipse: PaintEllipse,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ctx, |ui| {
           ui.heading("Hello Doodle Box!");
           
           ui.collapsing("Bézier Curve", |ui| {
               self.paint_bezier.ui_control(ui);
               self.paint_bezier.ui_content(ui);
           });
           
           ui.collapsing("Rectangle", |ui| {
               self.paint_rect.ui_control(ui);
               self.paint_rect.ui_content(ui);
           });

           ui.collapsing("Circle", |ui| {
               self.paint_circle.ui_control(ui);
               self.paint_circle.ui_content(ui);
           });

           ui.collapsing("Ellipse", |ui| {
               self.paint_ellipse.ui_control(ui);
               self.paint_ellipse.ui_content(ui);
           });
       });
   }
}