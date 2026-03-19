use eframe::egui;

use crate::vertices_data::VerticesData;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

pub struct EarClipperApp {
    vertices_data: VerticesData,
}

impl EarClipperApp {
    pub fn new() -> Self {
        Self {
            vertices_data: VerticesData::new(),
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
                if let Some(mouse_pos) = response.interact_pointer_pos() {
                    self.vertices_data.add_vertex(mouse_pos);
                }
            }

            let len = self.vertices_data.get_len();
            if len > 0 {
                let main_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);

                for i in 0..len {
                    let (start_point, end_point) = self.vertices_data.get_start_end_points(i);
                    painter.line_segment([start_point, end_point], main_stroke);
                    painter.circle_filled(start_point, 5.0, egui::Color32::RED);
                }

                if len >= 3 {
                    let triangles = self.vertices_data.triangulate();
                    let mesh_stroke =
                        egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255));

                    for tri in triangles {
                        painter.line_segment([tri[0], tri[1]], mesh_stroke);
                        painter.line_segment([tri[1], tri[2]], mesh_stroke);
                        painter.line_segment([tri[2], tri[0]], mesh_stroke);
                    }
                }
            }
        });
    }
}

// ============= private helpers =============
impl EarClipperApp {
    fn draw_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("tools").show(ctx, |ui| {
            let remove_vertex_button = ui.button("Remove Vertex");
            if remove_vertex_button.clicked() {
                self.vertices_data.pop();
            }

            let clear_button = ui.button("Clear");
            if clear_button.clicked() {
                self.vertices_data.clear();
            }
        });
    }
}
