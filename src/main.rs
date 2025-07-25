//! # waveflow-img
//!
//! `waveflow-img` is a simple tool for generating 2D maps using Wave Function Collapse.
//!
//! ## Features
//! - Generates images based on luminance-level adjacency rules
//! - Uses a YAML config
//! - Directional propagation with WFC
//!
//! ## Usage
//! ```sh
//! waveflow-img example.yaml output.png
//! ```

use clap::{Arg, Command};
use std::collections::HashMap;
use image::RgbaImage;

use crate::config::Config;

mod config;
mod utils;
mod wfc;

fn load_image(path: &str) -> RgbaImage {
    let img = image::open(path).expect(&format!("Failed to open image: {}", path));
    img.to_rgba8()
}

/// Maps RGBA colors to unique u32 IDs and returns the grid and color map
pub fn build_color_map(img: &RgbaImage) -> (Vec<Vec<u32>>, HashMap<[u8; 4], u32>) {
    let mut color_map = HashMap::new();
    let mut next_id = 0;
    let mut grid = Vec::new();

    for y in 0..img.height() {
        let mut row = Vec::new();
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y).0;
            let entry = color_map.entry(pixel).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            });
            row.push(*entry);
        }
        grid.push(row);
    }

    (grid, color_map)
}

/// Returns a 2D-grid with luminance levels per pixel (1..=luminance_levels)
pub fn image_to_luminance_grid(img: &RgbaImage, levels: u8) -> Vec<Vec<u8>> {
    let mut luminances = Vec::new();
    let mut flat_values = Vec::new();

    for y in 0..img.height() {
        let mut row = Vec::new();
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y).0;
            // ITU-R BT.709-luminance: 0.2126*R + 0.7152*G + 0.0722*B
            let lum = 0.2126 * pixel[0] as f32
                    + 0.7152 * pixel[1] as f32
                    + 0.0722 * pixel[2] as f32;
            row.push(lum);
            flat_values.push(lum);
        }
        luminances.push(row);
    }

    // Determine edges for levels
    let min = flat_values.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = flat_values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let step = (max - min) / levels as f32;

    // Convert to levels (1 to luminance_levels)
    luminances.iter()
        .map(|row| {
            row.iter()
                .map(|&lum| {
                    let idx = ((lum - min) / step).floor() as u8 + 1;
                    idx.min(levels) // make sure max = luminance_levels
                })
                .collect()
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("waveflow")
        .version("0.1.0")
        .author("Neo Mannskär")
        .about("A simpler tool for generating 2D game top-down maps through wave function collapse")
        .arg(
            Arg::new("input")
                .help("The input .yaml file")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("The output file for generated map")
                .required(false)
                .index(2),
        )
        .get_matches();

    let input = matches.get_one::<String>("input").expect("Input .yaml file is required");
    let output = matches.get_one::<String>("output").expect("Output .png file is required");

    println!("waveflow-img {} {}", input, output);

    let config = Config::from_yaml(input)?;

    let img = load_image(&config.input_image);
    
    let lum_grid = image_to_luminance_grid(&img, config.luminance_levels);
    
    let adjacency = utils::extract_directional_rules(&lum_grid);

    let result_grid = wfc::generate_output(&config, &adjacency);

    let result_img = utils::grid_to_image(&result_grid, config.luminance_levels);

    result_img.save(&output).expect("Failed to save image!");

    Ok(())
}
