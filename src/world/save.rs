use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use crate::world::WorldConfig;
use bevy::prelude::*;
use std::collections::HashMap;

const SAVE_DIR: &str = "./saved_maps";

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldSaveData {
    pub height_map: Vec<Vec<f32>>,
    pub moisture_map: Vec<Vec<f32>>,
    pub config: WorldConfig,
    pub version: String,
}

impl WorldSaveData {
    pub fn new(height_map: Vec<Vec<f32>>, moisture_map: Vec<Vec<f32>>, config: &WorldConfig) -> Self {
        Self {
            height_map,
            moisture_map,
            config: config.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        // Create save directory if it doesn't exist
        if !Path::new(SAVE_DIR).exists() {
            fs::create_dir_all(SAVE_DIR)?;
        }

        let path = format!("{}/{}.map", SAVE_DIR, filename);
        let serialized = bincode::serialize(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        fs::write(path, &serialized)?;
        Ok(())
    }

    pub fn load_from_file(filename: &str) -> std::io::Result<Self> {
        let path = format!("{}/{}.map", SAVE_DIR, filename);
        let data = fs::read(path)?;
        
        bincode::deserialize(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    pub fn list_saved_maps() -> std::io::Result<Vec<String>> {
        if !Path::new(SAVE_DIR).exists() {
            return Ok(Vec::new());
        }

        let mut maps = Vec::new();
        for entry in fs::read_dir(SAVE_DIR)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("map") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    maps.push(stem.to_string());
                }
            }
        }
        Ok(maps)
    }
}

#[derive(Resource)]
pub struct MapGenerationState {
    pub height_map: Option<Vec<Vec<f32>>>,
    pub moisture_map: Option<Vec<Vec<f32>>>,
    pub is_generating: bool,
    pub save_name: Option<String>,
}

impl Default for MapGenerationState {
    fn default() -> Self {
        Self {
            height_map: None,
            moisture_map: None,
            is_generating: false,
            save_name: None,
        }
    }
}
