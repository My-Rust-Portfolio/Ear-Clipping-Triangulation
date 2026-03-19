use eframe::egui;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub struct EarClipperApp {
    vertices: Vec<egui::Pos2>,
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
        self.draw_left_panel(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Click to add vertex. Hold to add continiously.");

            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::click_and_drag());

            if response.dragged() || response.clicked() {
                // left click
                if let Some(mouse_pos) = response.interact_pointer_pos() {
                    // min distance between consequative points
                    let min_distance = 30.0;

                    let should_add = match self.vertices.last() {
                        Some(&last_pos) => last_pos.distance(mouse_pos) > min_distance,
                        // always add the first point if empty
                        None => true,
                    };

                    if should_add {
                        self.vertices.push(mouse_pos);
                    }
                }
            }

            let stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
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
                painter.circle_filled(vertex, 5.0, egui::Color32::RED);
            }
        });
    }
}

impl EarClipperApp {
    fn draw_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("tools").show(ctx, |ui| {
            let remove_vertex_button = ui.button("Remove Vertex");
            if remove_vertex_button.clicked() {
                self.vertices.pop();
            }

            let clear_button = ui.button("Clear");
            if clear_button.clicked() {
                self.vertices.clear();
            }
        });
    }
}
