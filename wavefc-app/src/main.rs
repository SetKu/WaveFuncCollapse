use eframe::{egui, epaint::{vec2, Color32}};

const APP_NAME: &str = "Wave Function Collapse";

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(800.0, 600.0)),
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(APP_NAME, options, Box::new(|_cc| Box::new(WFCApp::default())));  
}

struct WFCApp {
    dropped_files: Vec<egui::DroppedFile>,
    image_source_path: Option<String>,
}

impl Default for WFCApp {
    fn default() -> Self {
        Self {
            dropped_files: vec![],
            image_source_path: None,
        }
    }
}

impl eframe::App for WFCApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(Color32::WHITE);
            ui.heading(egui::RichText::new("Wave Function Collapse"));
            ui.label("By SetKu");
            ui.add_space(10.);

            if ui.button("Import Source Image").clicked() {
                println!("clicked");
            }
        });
    }
}
