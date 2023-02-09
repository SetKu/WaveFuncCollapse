use super::DEFAULT_MAX_CONTRADICTIONS;
use crate::shared::SharedArgs;
use chrono::Local;
use clap::ArgMatches;
use image::io::Reader as ImageReader;
use image::ImageBuffer;
use image::Rgba;
use open::that;
use std::collections::HashMap;
use std::fs::canonicalize;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use wavefc::prelude::*;

const EXTRA_THREAD_COUNT: usize = 4;

pub fn handler(matches: &ArgMatches) -> Result<(), String> {
    let pathbuf = matches.get_one::<PathBuf>("sample").unwrap();
    let output = matches.get_one::<PathBuf>("output");
    let open = matches.get_flag("open");

    let shared_args = SharedArgs::from(matches);

    let preparation_start = Instant::now();

    let image_result = ImageReader::open(pathbuf.as_path().to_str().unwrap());
    let image = image_result
        .map_err(|e| format!("The image path provided was invalid: {}", e.to_string()))?
        .decode()
        .expect("Unable to decode the provided image");
    let width = image.width();
    let height = image.height();

    println!("Sample has the dimensions {} by {}.", width, height);

    let casted = image.into_rgba8();
    let mut source_map = HashMap::new();
    let mut id_counter: usize = 0;
    let mut bit_sample: Vec<Vec<usize>> = vec![];

    for x in 0..width {
        if (bit_sample.len() as u32) < x + 1 {
            bit_sample.push(vec![]);
        }

        for y in 0..height {
            let pixel = casted.get_pixel(x, y);

            source_map.entry(pixel.0).or_insert_with(|| {
                let v = id_counter;
                id_counter += 1;
                v
            });

            bit_sample[x as usize].push(source_map[&pixel.0]);
        }
    }

    let preparation_duration = preparation_start.elapsed();

    println!("Found {} unique colors.", source_map.len());

    let chunk_size = if shared_args.tilesize.is_some() {
        let mut size = Vector2::new(
            *shared_args.tilesize.unwrap(),
            *shared_args.tilesize.unwrap(),
        );

        if let Some(width) = shared_args.tilewidth {
            size.x = *width;
        }

        if let Some(height) = shared_args.tileheight {
            size.y = *height;
        }

        size
    } else {
        Vector2::new(1, 1)
    };

    let mut wave = Wave::new();

    if !shared_args.use_transforms {
        wave.flags.push(Flags::NoTransforms);
    }

    if !shared_args.use_weights {
        wave.flags.push(Flags::NoWeights);
    }

    wave.flags.push(Flags::NoHistory);

    let analysis_start = Instant::now();
    println!("Analyzing... (this could take some time)");
    wave.analyze(bit_sample, chunk_size, BorderMode::Clamp);
    let analysis_duration = analysis_start.elapsed();
    println!("Finished analyzing.");

    wave.fill(Vector2::new(shared_args.width, shared_args.height))?;

    let real_contradictions = if let Some(max) = shared_args.max_contradictions {
        *max
    } else {
        DEFAULT_MAX_CONTRADICTIONS
    };

    let mut waves = vec![];
    waves.reserve(EXTRA_THREAD_COUNT);

    for _ in 0..EXTRA_THREAD_COUNT {
        waves.push(wave.clone());
    }

    let finished = Arc::new(Mutex::new(false));
    let mut thread_handles = vec![];
    let collapse_start = Instant::now();

    for (i, mut wave) in waves.into_iter().enumerate() {
        let finished_ref_copy = finished.clone();

        thread_handles.push(thread::spawn(move || {
            let midway_print = Some(
                |iterations: usize, failures: usize, _: Vec<Vec<Vec<usize>>>| {
                    let finished_local = finished_ref_copy.lock().unwrap();

                    if !*finished_local {
                        println!(
                            "Thread {}: Currently on attempt {} iteration {}",
                            i + 1,
                            failures + 1,
                            iterations + 1
                        );
                    }
                },
            );

            println!("Thread {}: Currently on attempt 1 iteration 1", i + 1);
            wave.collapse_all(real_contradictions, midway_print)
                .map(|_| wave)
        }));
    }

    let success: Option<Wave>;
    let collapse_duration: Duration;

    'outer: loop {
        if thread_handles.is_empty() {
            return Err("Failed to find result. The max numbe of contradictions has been reached on all threads.".to_string());
        }

        let finished_threads_indexes: Vec<usize> = thread_handles
            .iter()
            .enumerate()
            .filter_map(|(i, h)| if h.is_finished() { Some(i) } else { None })
            .collect();

        if finished_threads_indexes.is_empty() {
            thread::sleep(Duration::from_millis(250));
            continue;
        }

        let mut removed = 0;

        for index in finished_threads_indexes {
            let handle = thread_handles.remove(index - removed);
            removed += 1;

            let result = handle.join().expect("Failed to join threads.");

            if let Ok(wave) = result {
                // A successful collapse was made here
                collapse_duration = collapse_start.elapsed();
                success = Some(wave);
                *finished.lock().unwrap() = true;
                break 'outer;
            }
        }
    }

    debug_assert!(success.is_some());

    let result = success.unwrap().perfect_rep()?;
    let mut result_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(shared_args.width as u32, shared_args.height as u32);

    std::mem::drop(shared_args);

    let dimensions = result_buffer.dimensions();

    for x in 0..dimensions.0 {
        for y in 0..dimensions.1 {
            let sample_id = result[x as usize][y as usize];
            let pixel_data = source_map
                .iter()
                .find_map(|(key, &val)| if val == sample_id { Some(key) } else { None })
                .unwrap();
            let copy = pixel_data.clone();
            result_buffer.put_pixel(x, y, Rgba::from(copy));
        }
    }

    let output_pathbuf = if let Some(filename) = output {
        filename.to_owned()
    } else {
        let mut path = pathbuf.to_owned();
        let file_name = pathbuf.as_path().file_name().unwrap().to_str().unwrap();
        let time_string = Local::now().format("%Y-%m-%dT%H-%M-%S").to_string();
        path.set_file_name(time_string + " " + file_name);
        path
    };

    result_buffer
        .save(output_pathbuf.as_path())
        .map_err(|e| e.to_string())?;

    println!("");
    println!("Saved result to {}", output_pathbuf.to_str().unwrap());

    println!("\nPreparation Time: {:?}", preparation_duration);
    println!("Analysis Time: {:?}", analysis_duration);
    println!("Collapse Time: {:?}", collapse_duration);

    if open {
        let absolute_path =
            canonicalize(output_pathbuf.to_str().unwrap()).map_err(|e| e.to_string())?;
        println!("\nOpening... {}", absolute_path.to_str().unwrap());
        that(absolute_path).map_err(|e| e.to_string())?;
    }

    println!("\nWaiting for other threads to finish...\n");

    Ok(())
}
