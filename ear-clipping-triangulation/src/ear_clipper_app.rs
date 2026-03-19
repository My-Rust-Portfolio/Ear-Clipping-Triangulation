use eframe::egui;
use egui::{Color32, Pos2, Sense, Stroke};

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub struct EarClipperApp {
    vertices: Vec<Pos2>,
}

impl EarClipperApp {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }

    pub fn run() -> eframe::Result<()> {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT]),
            ..Default::default()
        };

        eframe::run_native(
            "Ear Clipper",
            options,
            Box::new(|_cc| Ok(Box::new(EarClipperApp::new()))),
        )
    }
}

impl eframe::App for EarClipperApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Left click to add vertices. Right click to clear.");

            let (response, painter) = ui.allocate_painter(ui.available_size(), Sense::click());

            // left click
            if response.clicked() {
                if let Some(mouse_pos) = response.interact_pointer_pos() {
                    self.vertices.push(mouse_pos);
                }
            }

            // right click
            if response.secondary_clicked() {
                self.vertices.clear();
            }

            let stroke = Stroke::new(2.0, Color32::WHITE);
            if self.vertices.len() > 1 {
                for i in 0..self.vertices.len() {
                    let start_point = self.vertices[i];
                    // connect the last point back to the first point to close the shape
                    let end_point = self.vertices[(i + 1) % self.vertices.len()];
                    painter.line_segment([start_point, end_point], stroke);
                }
            }

            // draw the vertices
            for &vertex in &self.vertices {
                painter.circle_filled(vertex, 5.0, Color32::RED);
            }
        });
    }
}
