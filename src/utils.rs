use std::collections::HashMap;

use image::{Rgba, RgbaImage};

use crate::config::DirectionalRules;

pub fn grid_to_image(grid: &[Vec<u8>], levels: u8) -> RgbaImage {
    let mut img = RgbaImage::new(grid[0].len() as u32, grid.len() as u32);

    for (y, row) in grid.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let gray = (255.0 * (val as f32 / levels as f32)) as u8;
            img.put_pixel(x as u32, y as u32, Rgba([gray, gray, gray, 255]));
        }
    }

    img
}

/// Extracts directional adjacency rules (Vec<u8>) from a grid of u8 levels
pub fn extract_directional_rules(grid: &[Vec<u8>]) -> HashMap<u8, DirectionalRules> {
    let height = grid.len();
    let width = grid[0].len();
    let mut rules: HashMap<u8, DirectionalRules> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let current = grid[y][x];

            let entry = rules.entry(current).or_insert_with(|| DirectionalRules {
                top: Vec::new(),
                bottom: Vec::new(),
                left: Vec::new(),
                right: Vec::new(),
            });

            if y > 0 {
                let neighbor = grid[y - 1][x];
                if !entry.top.contains(&neighbor) {
                    entry.top.push(neighbor);
                }
            }
            if y + 1 < height {
                let neighbor = grid[y + 1][x];
                if !entry.bottom.contains(&neighbor) {
                    entry.bottom.push(neighbor);
                }
            }
            if x > 0 {
                let neighbor = grid[y][x - 1];
                if !entry.left.contains(&neighbor) {
                    entry.left.push(neighbor);
                }
            }
            if x + 1 < width {
                let neighbor = grid[y][x + 1];
                if !entry.right.contains(&neighbor) {
                    entry.right.push(neighbor);
                }
            }
        }
    }

    rules
}
