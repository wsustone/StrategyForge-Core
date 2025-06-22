use bevy::prelude::*;
use bevy::log::{info, warn};
// Using Bevy's built-in noise functions instead of external crate
use std::f32::consts::PI;
use std::time::Instant;
use super::{
    WorldConfig, GameWorld, Commands, Name, SpatialBundle, GlobalTransform, Transform,
    TerrainType, TerrainTile
};

// TerrainType and TerrainTile are now defined in mod.rs and re-exported

/// Spawns the initial game world
pub fn spawn_initial_world(
    mut commands: Commands,
    config: ResMut<WorldConfig>,
) {
    info!("Starting world generation...");
    let _start_time = Instant::now();
    
    // Spawn the main world entity
    info!("Spawning main world entity...");
    let world_entity = commands.spawn((
        GameWorld,
        Name::new("Game World"),
        SpatialBundle::default(),
    )).id();
    
    // Generate terrain
    info!("Generating height map...");
    let height_map = generate_height_map(&config);
    info!("Generating moisture map...");
    let moisture_map = generate_moisture_map(&height_map, config.seed);
    
    // Calculate dimensions in tiles
    let width_tiles = config.width_tiles();
    let height_tiles = config.height_tiles();
    info!("World dimensions: {}x{} tiles", width_tiles, height_tiles);
    
    // Spawn terrain tiles
    info!("Spawning terrain tiles...");
    let total_tiles = (width_tiles * height_tiles) as usize;
    let mut tiles_spawned = 0;
    let report_interval = (total_tiles / 10).max(1); // Report every 10%
    
    let _tile_spawn_start = Instant::now();
    for y in 0..height_tiles {
        for x in 0..width_tiles {
            let height = height_map[y as usize][x as usize];
            let moisture = moisture_map[y as usize][x as usize];
            let terrain_type = determine_terrain_type(&config, height, moisture);
            
            // Calculate world position based on tile size
            let pos_x = x as f32 * config.tile_size + config.border_width;
            let pos_y = y as f32 * config.tile_size + config.border_width;
            
            let tile_entity = commands.spawn((
                TerrainTile {
                    terrain_type,
                    height,
                    moisture,
                },
                Transform::from_xyz(pos_x, pos_y, 0.0),
                GlobalTransform::default(),
            )).id();
            
            // Report progress
            tiles_spawned += 1;
            if tiles_spawned % report_interval == 0 {
                let progress = (tiles_spawned as f32 / total_tiles as f32) * 100.0;
                info!("Spawning tiles: {}/{} ({:.1}%)", tiles_spawned, total_tiles, progress);
            }
            
            commands.entity(world_entity).add_child(tile_entity);
        }
    }
    
    // Store world dimensions for reference
    commands.insert_resource(WorldDimensions {
        width: width_tiles,
        height: height_tiles,
        tile_size: config.tile_size,
        border_width: config.border_width,
    });
}

/// Resource storing the world dimensions
#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldDimensions {
    pub width: u32,
    pub height: u32,
    pub tile_size: f32,
    pub border_width: f32,
}

/// Generates a moisture map based on height map and noise
fn generate_moisture_map(height_map: &[Vec<f32>], seed: u32) -> Vec<Vec<f32>> {
    info!("Starting moisture map generation...");
    let start_time = Instant::now();
    
    let height = height_map.len();
    if height == 0 {
        warn!("Empty height map provided for moisture generation");
        return Vec::new();
    }
    let width = height_map[0].len();
    info!("Generating moisture map of size {}x{}", width, height);
    
    let mut moisture_map = vec![vec![0.0; width]; height];
    
    let total_pixels = width * height;
    let mut pixels_processed = 0;
    let report_interval = (total_pixels / 10).max(1); // Report every 10%
    
    info!("Processing moisture map pixels...");
    for y in 0..height {
        for x in 0..width {
            // Base moisture on height (lower areas are wetter)
            let height = height_map[y][x];
            let mut moisture = 1.0 - height; // Invert height for moisture
            
            // Generate simple noise using sine waves
            let nx = x as f32 / width as f32 * 5.0;
            let ny = y as f32 / height as f32 * 5.0;
            
            // Simple 2D noise using sine waves
            let noise = ((nx * 2.0 * PI).sin() * 0.3 + (ny * 2.0 * PI).sin() * 0.3 + 1.0) * 0.5;
            
            // Add some randomness based on position and seed
            let rand = ((x as u32).wrapping_mul(seed) ^ (y as u32).wrapping_mul(seed.wrapping_mul(1664525).wrapping_add(1013904223))) as f32 / u32::MAX as f32;
            
            // Combine height-based moisture with noise
            moisture = (moisture * 0.7 + noise * 0.15 + rand * 0.15).clamp(0.0, 1.0);
            moisture_map[y][x] = moisture;
            
            // Report progress
            pixels_processed += 1;
            if pixels_processed % report_interval == 0 {
                let progress = (pixels_processed as f32 / total_pixels as f32) * 100.0;
                info!("Moisture map: {}/{} pixels processed ({:.1}%)", pixels_processed, total_pixels, progress);
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    info!("Moisture map generation completed in {:.2?}", elapsed);
    
    moisture_map
}

/// Determines terrain type based on height and moisture
fn determine_terrain_type(config: &WorldConfig, height: f32, moisture: f32) -> TerrainType {
    match () {
        _ if height < config.water_level => TerrainType::Water,
        _ if height > config.mountain_level => TerrainType::Mountains,
        _ if moisture > config.forest_moisture => TerrainType::Forest,
        _ if moisture < config.desert_moisture => TerrainType::Desert,
        _ => TerrainType::Grassland,
    }
}

/// Generates a height map for terrain generation using multiple octaves of noise
fn generate_height_map(config: &WorldConfig) -> Vec<Vec<f32>> {
    info!("Starting height map generation...");
    let start_time = Instant::now();
    
    // Get dimensions in tiles
    let width = config.width_tiles() as usize;
    let height = config.height_tiles() as usize;
    info!("Generating height map of size {}x{}", width, height);
    
    // Create a grid of noise values
    let mut height_map = vec![vec![0.0; width]; height];
    
    // Use a simple sine-based noise function
    let seed = if config.seed == 0 { 42 } else { config.seed };
    
    let total_pixels = width * height;
    let mut pixels_processed = 0;
    let report_interval = (total_pixels / 10).max(1); // Report every 10%
    
    info!("Processing height map pixels...");
    for y in 0..height {
        for x in 0..width {
            // Normalize coordinates to 0..1 range
            let nx = x as f32 / width as f32 * 10.0;
            let ny = y as f32 / height as f32 * 10.0;
            
            // Simple 2D noise using sine waves
            let mut value = (nx * 2.0 * PI).sin() * 0.3 + (ny * 2.0 * PI).sin() * 0.3;
            
            // Add some randomness based on position and seed
            let rand = ((x as u32).wrapping_mul(seed) ^ (y as u32).wrapping_mul(seed.wrapping_mul(1664525).wrapping_add(1013904223))) as f32 / u32::MAX as f32;
            
            // Combine noise with randomness
            value = (value * 0.7 + rand * 0.3).clamp(-1.0, 1.0);
            
            // Apply falloff to create an island
            let dx = (x as f32 / width as f32 * 2.0 - 1.0).abs();
            let dy = (y as f32 / height as f32 * 2.0 - 1.0).abs();
            let d = (dx * dx + dy * dy).sqrt();
            let falloff = 1.0 - (d * 1.4).clamp(0.0, 1.0).powi(2);
            
            // Apply falloff and normalize to 0..1 range
            height_map[y][x] = ((value * falloff + 1.0) / 2.0).clamp(0.0, 1.0);
            
            // Report progress
            pixels_processed += 1;
            if pixels_processed % report_interval == 0 {
                let progress = (pixels_processed as f32 / total_pixels as f32) * 100.0;
                info!("Height map: {}/{} pixels processed ({:.1}%)", pixels_processed, total_pixels, progress);
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    info!("Height map generation completed in {:.2?}", elapsed);
    
    height_map
}
