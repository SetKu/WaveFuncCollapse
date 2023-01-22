#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui,
    epaint::{vec2, Color32, ColorImage},
};
use egui_extras::RetainedImage;
use std::{path::PathBuf, thread, sync::{Mutex, Arc}};

const APP_NAME: &str = "Wave Function Collapse";

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(800.0, 600.0)),
        drag_and_drop_support: true,
        ..Default::default()
    };

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(WFCApp::default())),
    );
}

struct WFCApp {
    image_path: Option<PathBuf>,
    image_retainer: Arc<Mutex<Option<RetainedImage>>>,
}

impl Default for WFCApp {
    fn default() -> Self {
        Self {
            image_path: None,
            image_retainer: Arc::new(Mutex::new(None)),
        }
    }
}

impl WFCApp {
    fn background_update_retainer(&self, path: PathBuf) {
        let arc_copy = self.image_retainer.clone();

        thread::spawn(move || {
            let retainer = updated_image_retainer(path);
            
            if let Ok(mut reference) = arc_copy.lock() {
                *reference = retainer;
            }
        });
    }
}

impl eframe::App for WFCApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Increases global UI scale but causes
            // half-second blank screen on resize and start.
            // ctx.set_pixels_per_point(2.);

            ui.visuals_mut().override_text_color = Some(Color32::WHITE);

            let width = ui.available_width();

            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.set_max_width(width / 2.);
                    
                    ui.heading(egui::RichText::new("Wave Function Collapse"));
                    ui.label("By SetKu");
                    ui.add_space(10.);

                    if ui.button("Import Source Image").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.image_path = Some(path.clone());
                            self.background_update_retainer(path);
                        }
                    }
        
                    ui.label("You can also drag and drop the image into the window.");
                    ui.add_space(10.);

                    if let Some(path) = &self.image_path {
                        ui.label(format!("Selected Image Source: {}", path.display()));
        
                        if let Ok(ret_lock) = self.image_retainer.try_lock() {
                            if let Some(ret) = ret_lock.as_ref() {
                                ret.show_max_size(ui, vec2(300., 300.));
                            }
                        }
                    }
                });

                ui.separator();

                ui.vertical(|ui| {
                    
                });
            });
        });

        if !ctx.input().raw.dropped_files.is_empty() {
            let first = ctx.input().raw.dropped_files.first().unwrap().clone();
            self.image_path = first.path.clone();
            
            if let Some(path) = first.path {
                self.background_update_retainer(path);
            }
        }
    }
}

fn updated_image_retainer(path: PathBuf) -> Option<RetainedImage> {
    if let Ok(image) = image::io::Reader::open(path) {
        if let Ok(decoded) = image.decode() {
            let data = decoded.to_rgba8();
            let pixels = data.as_flat_samples().samples;
            let paint_image = ColorImage::from_rgba_unmultiplied(
                [decoded.width() as _, decoded.height() as _],
                pixels,
            );
                
            return Some(RetainedImage::from_color_image("chosen-image", paint_image));
        }
    }

    return None;
}