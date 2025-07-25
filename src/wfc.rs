use std::collections::{HashMap, HashSet, VecDeque};

use rand::seq::IteratorRandom;
use rand::rng;

use crate::Config;

/// Wave Function Collapse
pub fn generate_output(
    config: &Config,
    adjacency: &HashMap<u8, crate::config::DirectionalRules>,
) -> Vec<Vec<u8>> {
    let width = (config.output_width * config.tile_size) as usize;
    let height = (config.output_height * config.tile_size) as usize;
    let all_levels: HashSet<u8> = (1..=config.luminance_levels).collect();

    // Each cell begins with all possible levels
    let mut possibilities: Vec<Vec<HashSet<u8>>> = vec![
        vec![all_levels.clone(); width];
        height
    ];

    // As long as there are uncollapsed cells
    while let Some((x, y)) = find_least_entropy_cell(&possibilities) {
        let options = &possibilities[y][x];
        if options.is_empty() {
            panic!("Collapse conflict no valid alternatives for ({x}, {y})");
        }

        // Pick one of the possible levels at random
        let choice = *options.iter().choose(&mut rng()).unwrap();
        possibilities[y][x] = HashSet::from([choice]);

        // Propagate to neighbors
        propagate(&mut possibilities, x, y, &adjacency);
    }

    // Convert to u8-grid
    possibilities
        .into_iter()
        .map(|row| row.into_iter().map(|s| *s.iter().next().unwrap()).collect())
        .collect()
}

/// Find cell with lowest entropy (>1 alternative)
fn find_least_entropy_cell(grid: &[Vec<HashSet<u8>>]) -> Option<(usize, usize)> {
    let mut min = usize::MAX;
    let mut candidates = vec![];

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let len = cell.len();
            if len > 1 && len < min {
                min = len;
                candidates = vec![(x, y)];
            } else if len == min {
                candidates.push((x, y));
            }
        }
    }

    candidates.into_iter().choose(&mut rng())
}

/// Propagate constraints to neighbors
fn propagate(
    grid: &mut [Vec<HashSet<u8>>],
    x: usize,
    y: usize,
    adjacency: &HashMap<u8, crate::config::DirectionalRules>,
) {
    let width = grid[0].len();
    let height = grid.len();
    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    while let Some((cx, cy)) = queue.pop_front() {
        let current = &grid[cy][cx].clone(); // nuvarande möjligheter

        for (dx, dy, dir) in &[
            (0, -1, "top"),
            (0, 1, "bottom"),
            (-1, 0, "left"),
            (1, 0, "right"),
        ] {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx < 0 || ny < 0 || nx >= width as isize || ny >= height as isize {
                continue;
            }

            let (nx, ny) = (nx as usize, ny as usize);
            let mut new_possibilities = HashSet::new();

            for &own_val in current {
                if let Some(rule) = adjacency.get(&own_val) {
                    let allowed = match *dir {
                        "top" => &rule.top,
                        "bottom" => &rule.bottom,
                        "left" => &rule.left,
                        "right" => &rule.right,
                        _ => continue,
                    };
                    new_possibilities.extend(allowed.iter().copied());
                }
            }

            let cell = &mut grid[ny][nx];
            let before = cell.len();
            *cell = cell.intersection(&new_possibilities).cloned().collect();
            if cell.len() < before {
                queue.push_back((nx, ny));
            }
        }
    }
}
