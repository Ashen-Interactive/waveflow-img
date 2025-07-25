use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DirectionalRules {
    pub top: Vec<u8>,
    pub bottom: Vec<u8>,
    pub left: Vec<u8>,
    pub right: Vec<u8>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub input_image: String,
    pub output_width: u32,
    pub output_height: u32,
    pub tile_size: u32,
    pub luminance_levels: u8,

    // Add later somehow
    // pub adjacency: HashMap<u8, DirectionalRules>,
}

impl Config {
    pub fn from_yaml(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml_str = fs::read_to_string(file_path)?;
        let tile: Config = serde_yaml::from_str(&yaml_str)?;
        Ok(tile)
    }
}
