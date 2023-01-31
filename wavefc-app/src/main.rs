#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{
    egui,
    egui::{Button, Frame, Label, Layout, Spinner, WidgetText},
    emath::Align,
    epaint::{vec2, Color32, ColorImage, Rect, Rounding, Vec2},
};
use egui_extras::RetainedImage;
use image::{ImageBuffer, Rgba};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread, collections::HashMap,
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
    image_data: Arc<Mutex<Option<ImageBuffer<Rgba<u8>, Vec<u8>>>>>,
    image_retainer: Arc<Mutex<Option<RetainedImage>>>,
    loading_image: Arc<Mutex<bool>>,
    wave: Arc<Mutex<Wave>>,
    chunk_size: Vector2<usize>,
    stop_signal: Arc<Mutex<bool>>,
    analysis_source_map: Arc<Mutex<HashMap<[u8; 4], usize>>>,
    analyzing: Arc<Mutex<bool>>,
    collapsing: Arc<Mutex<bool>>,
}

impl Default for WFCApp {
    fn default() -> Self {
        let mut wave = Wave::new();
        wave.flags.append(&mut vec![Flags::NoHistory]);

        Self {
            image_path: None,
            image_data: Arc::new(Mutex::new(None)),
            image_retainer: Arc::new(Mutex::new(None)),
            loading_image: Arc::new(Mutex::new(false)),
            wave: Arc::new(Mutex::new(wave)),
            chunk_size: Vector2::new(1, 1),
            stop_signal: Arc::new(Mutex::new(false)),
            analysis_source_map: Arc::new(Mutex::new(HashMap::new())),
            analyzing: Arc::new(Mutex::new(false)),
            collapsing: Arc::new(Mutex::new(false)),
        }
    }
}

impl WFCApp {
    fn background_update_image(&self, path: PathBuf) {
        let i_arc_copy = self.image_retainer.clone();
        let b_arc_copy = self.loading_image.clone();
        let m_arc_copy = self.image_data.clone();

        *self.loading_image.lock().unwrap() = true;

        thread::spawn(move || {
            #[cfg(debug)]
            let started = std::time::Instant::now();

            #[cfg(debug)]
            println!("STARTED LOADING IMAGE");

            let results = load_image_info(path);

            if let Some(results) = results {
                if let Ok(mut reference) = i_arc_copy.lock() {
                    *reference = Some(results.1);
                }
    
                if let Ok(mut reference) = m_arc_copy.lock() {
                    *reference = Some(results.0);
                }
            } else {
                if let Ok(mut reference) = i_arc_copy.lock() {
                    *reference = None;
                }
    
                if let Ok(mut reference) = m_arc_copy.lock() {
                    *reference = None;
                }
            }

            #[cfg(debug)]
            println!(
                "OBTAINED IMAGE ({:?}): {:?}",
                started.elapsed(),
                retainer.is_some()
            );

            if let Ok(mut reference) = b_arc_copy.lock() {
                *reference = false;
            }
        });
    }

    fn background_collapse(&self) {
        let analyzing_ref = self.analyzing.clone();
        thread::spawn(move || {});
    }

    fn background_analyze(&self) {
        let image = self.image_data.lock().unwrap();

        if image.is_none() {
            return;
        }

        let image = image.as_ref().unwrap().clone();
        let chunk_size = self.chunk_size.clone();
        let analyzing_ref = self.analyzing.clone();

        let wave_ref = self.wave.clone();
        let map_ref = self.analysis_source_map.clone();

        thread::spawn(move || {
            let width = image.width();
            let height = image.height();

            *analyzing_ref.lock().unwrap() = true;

            // Convert each pixel in the image into a uniqued binary number by color.
            // Color image -> Binary image.

            let mut source_map = HashMap::<[u8; 4], usize>::new();
            let mut id_counter: usize = 0;
            let mut bit_sample: Vec<Vec<usize>> = vec![];

            for x in 0..width {
                if (bit_sample.len() as u32) < x + 1 {
                    bit_sample.push(vec![]);
                }

                for y in 0..height {
                    // Pixel is a tuple with one element.
                    let pixel = image.get_pixel(x, y).0;

                    source_map.entry(pixel).or_insert_with(|| {
                        let v = id_counter;
                        id_counter += 1;
                        v
                    });

                    bit_sample[x as usize].push(source_map[&pixel]);
                }
            }

            wave_ref
                .lock()
                .unwrap()
                .analyze(bit_sample, chunk_size, BorderMode::Clamp);

            let mut reference = map_ref.lock().unwrap();
            *reference = source_map;

            *analyzing_ref.lock().unwrap() = false;
        });
    }
}

impl eframe::App for WFCApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(Frame::none().fill(Color32::WHITE).inner_margin(15.))
            .show(ctx, |ui| {
                // Increases global UI scale but causes
                // half-second blank screen on resize and start.
                // ctx.set_pixels_per_point(2.);

                ui.visuals_mut().override_text_color = Some(Color32::BLACK);

                let width = ui.available_width();
                let height = ui.available_height();

                ui.horizontal_top(|ui| {
                    ui.with_layout(Layout::top_down(Align::Min), |ui| {
                        ui.set_min_width(width / 2.);
                        ui.set_max_width(width / 2.);

                        ui.heading(egui::RichText::new("Wave Function Collapse"));
                        ui.label("By SetKu");
                        ui.add_space(10.);

                        let button = Button::new(white_text("Import Source Image"));

                        if ui.add(button).clicked() {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                self.image_path = Some(path.clone());
                                self.background_update_image(path);
                            }
                        }

                        ui.label("You can also drag and drop the image into the window.");
                        ui.add_space(10.);

                        if let Some(path) = &self.image_path {
                            ui.label(format!("Selected Image Source: {}", path.display()));
                            ui.add_space(10.);

                            if *self.loading_image.lock().unwrap() {
                                ui.label("Loading image...");
                                ui.add(quick_spinner());
                            } else {
                                if let Ok(ret_lock) = self.image_retainer.try_lock() {
                                    if let Some(ret) = ret_lock.as_ref() {
                                        ret.show_max_size(ui, vec2(width / 5., height / 5.));
                                    }
                                }
                            }
                        }

                        ui.add_space(15.);

                        ui.group(|ui| {
                            ui.label("Chunk Size");

                            ui.horizontal_top(|ui| {
                                if ui.button(white_text("-")).clicked() {
                                    if self.chunk_size.x > 1 {
                                        self.chunk_size.x -= 1;
                                    }
                                }

                                ui.add_sized(
                                    vec2(30., 10.),
                                    Label::new(format!("x: {}", self.chunk_size.x)),
                                );

                                if ui.button(white_text("+")).clicked() {
                                    self.chunk_size.x += 1;
                                }
                            });

                            ui.horizontal_top(|ui| {
                                if ui.button(white_text("-")).clicked() {
                                    if self.chunk_size.y > 1 {
                                        self.chunk_size.y -= 1;
                                    }
                                }

                                ui.add_sized(
                                    vec2(30., 10.),
                                    Label::new(format!("y: {}", self.chunk_size.y)),
                                );

                                if ui.button(white_text("+")).clicked() {
                                    self.chunk_size.y += 1;
                                }
                            });
                        });

                        ui.horizontal(|ui| {
                            if ui.button(white_text("Analyze")).clicked() {
                                self.background_analyze();
                            }

                            if let Ok(reference) = self.analyzing.try_lock() {
                                if *reference {
                                    ui.add(quick_spinner());
                                }
                            }
                        });
                    });

                    ui.with_layout(Layout::top_down_justified(Align::Min), |ui| {
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
                self.background_update_image(path);
            }
        }
    }
}

fn load_image_info(path: PathBuf) -> Option<(ImageBuffer<Rgba<u8>, Vec<u8>>, RetainedImage)> {
    if let Ok(image) = image::io::Reader::open(path) {
        if let Ok(decoded) = image.decode() {
            #[cfg(debug)]
            println!("DECODED IMAGE SUCCESSFULLY");

            let data = decoded.to_rgba8();
            let pixels = data.as_flat_samples().samples;
            let paint_image = ColorImage::from_rgba_unmultiplied(
                [decoded.width() as _, decoded.height() as _],
                pixels,
            );

            #[cfg(debug)]
            println!("FINISHED INTERMEDIATE PROCESSING");

            return Some((
                data,
                RetainedImage::from_color_image("chosen-image", paint_image),
            ));
        }
    }

    return None;
}

fn white_text(input: &str) -> WidgetText {
    WidgetText::from(input).color(Color32::WHITE)
}

fn quick_spinner() -> Spinner {
    Spinner::default().size(20.).color(Color32::BLACK)
}
