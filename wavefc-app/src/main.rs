#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui,
    egui::{Button, FontDefinitions, Frame, WidgetText, Spinner},
    epaint::{vec2, Color32, ColorImage, Rect, Rounding, Stroke, Vec2},
};
use egui_extras::RetainedImage;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};
use wavefc::prelude::*;

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
    loading_image: Arc<Mutex<bool>>,
}

impl Default for WFCApp {
    fn default() -> Self {
        Self {
            image_path: None,
            image_retainer: Arc::new(Mutex::new(None)),
            loading_image: Arc::new(Mutex::new(false)),
        }
    }
}

impl WFCApp {
    fn background_update_retainer(&self, path: PathBuf) {
        let i_arc_copy = self.image_retainer.clone();
        let b_arc_copy = self.loading_image.clone();

        *self.loading_image.lock().unwrap() = true;

        thread::spawn(move || {
            let started = std::time::Instant::now();
            println!("STARTED LOADING IMAGE");
            let retainer = updated_image_retainer(path);
            println!("OBTAINED IMAGE ({:?}): {:?}", started.elapsed(), retainer.is_some());

            if let Ok(mut reference) = i_arc_copy.lock() {
                *reference = retainer;
            }

            if let Ok(mut reference) = b_arc_copy.lock() {
                *reference = false;
            }
        });
    }
}

impl eframe::App for WFCApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE).inner_margin(10.))
            .show(ctx, |ui| {
                // Increases global UI scale but causes
                // half-second blank screen on resize and start.
                // ctx.set_pixels_per_point(2.);

                ui.visuals_mut().override_text_color = Some(Color32::BLACK);

                let width = ui.available_width();
                let height = ui.available_height();

                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.set_min_width(width / 2.);
                        ui.set_max_width(width / 2.);

                        ui.heading(egui::RichText::new("Wave Function Collapse"));
                        ui.label("By SetKu");
                        ui.add_space(10.);

                        let button = Button::new(
                            WidgetText::from("Import Source Image").color(Color32::WHITE),
                        );

                        if ui.add(button).clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.image_path = Some(path.clone());
                                self.background_update_retainer(path);
                            }
                        }

                        ui.label("You can also drag and drop the image into the window.");
                        ui.add_space(10.);

                        if let Some(path) = &self.image_path {
                            ui.label(format!("Selected Image Source: {}", path.display()));
                            ui.add_space(10.);

                            if *self.loading_image.lock().unwrap() {
                                ui.label("Loading image...");
                                ui.add(Spinner::default().size(20.).color(Color32::BLACK));
                            } else {
                                if let Ok(ret_lock) = self.image_retainer.try_lock() {
                                    if let Some(ret) = ret_lock.as_ref() {
                                        ret.show_max_size(ui, vec2(width / 2., height / 2.));
                                    }
                                }
                            }
                        }
                    });

                    // ui.separator();

                    ui.vertical(|ui| {
                        ui.set_min_width(width / 2.);
                        ui.set_max_width(width / 2.);
                        let origin = ui.next_widget_position();

                        // Grid Background
                        ui.painter().rect_filled(
                            Rect::from_min_size(origin, Vec2::new(width / 2., height)),
                            Rounding::none(),
                            Color32::DARK_BLUE,
                        );

                        // Grid Square Template
                        const HORIZONTAL_TILES: u32 = 8;

                        ui.painter_at(Rect::from_min_size(origin, Vec2::new(width / 4., height)))
                            .rect_filled(
                                Rect::from_min_size(
                                    origin,
                                    Vec2::new(
                                        width / 2. / HORIZONTAL_TILES as f32,
                                        width / 2. / HORIZONTAL_TILES as f32,
                                    ),
                                ),
                                Rounding::none(),
                                Color32::GREEN,
                            );
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
            println!("DECODED IMAGE SUCCESSFULLY");
            let data = decoded.to_rgba8();
            let pixels = data.as_flat_samples().samples;
            let paint_image = ColorImage::from_rgba_unmultiplied(
                [decoded.width() as _, decoded.height() as _],
                pixels,
            );
            println!("FINISHED INTERMEDIATE PROCESSING");

            return Some(RetainedImage::from_color_image("chosen-image", paint_image));
        }
    }

    return None;
}
