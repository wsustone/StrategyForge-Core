//! World generation and management

mod border;
mod generation;
mod render;
mod resource_types;
mod resources;
mod terrain_render;

/// Represents different terrain types with their properties
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub enum TerrainType {
    Grassland,
    Forest,
    Mountains,
    Desert,
    Water,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType::Grassland
    }
}

/// Component for terrain tiles
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct TerrainTile {
    pub terrain_type: TerrainType,
    pub height: f32,
    pub moisture: f32,
}

// Re-exports
pub use border::{MapBorder, MapBorderPlugin};
pub use render::ResourceRenderPlugin;
pub use resource_types::{ResourceType, ResourceNode};
pub use resources::ResourceNodeMarker;
pub use terrain_render::TerrainRenderPlugin;

use bevy::prelude::*;
use bevy::reflect::Reflect;
use crate::state::GameState;

/// Plugin for world-related functionality
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        // Register components for reflection
        app.register_type::<TerrainTile>()
           .register_type::<ResourceNode>()
           .register_type::<MapBorder>()
           .register_type::<ResourceNodeMarker>()
           .register_type::<WorldConfig>()
           .init_resource::<WorldConfig>()
           .add_plugins((
               MapBorderPlugin,
               TerrainRenderPlugin,
               ResourceRenderPlugin,
           ));
        
        // Add systems
        app.add_systems(OnEnter(GameState::InGame { is_paused: false }), (
            generation::spawn_initial_world,
            resources::spawn_initial_resources.after(generation::spawn_initial_world),
        ))
        .add_systems(OnExit(GameState::InGame { is_paused: false }), cleanup_world);
    }
}

/// Component marking the main game world
#[derive(Component)]
pub struct GameWorld;

/// Cleans up all world entities when exiting the game state
fn cleanup_world(
    mut commands: Commands,
    world_query: Query<Entity, With<GameWorld>>,
    tile_query: Query<Entity, (With<TerrainTile>, Without<GameWorld>)>,
) {
    // Despawn the main world entity
    for entity in &world_query {
        commands.entity(entity).despawn_recursive();
    }
    
    // Despawn all tile entities
    for entity in &tile_query {
        commands.entity(entity).despawn_recursive();
    }
}

/// Resource containing world generation settings
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct WorldConfig {
    /// Width of the world in meters
    pub width_meters: f32,
    /// Height of the world in meters
    pub height_meters: f32,
    /// Size of each tile in meters (tiles are square)
    pub tile_size: f32,
    /// Width of the map border in meters
    pub border_width: f32,
    /// Random seed for world generation
    pub seed: u32,
    /// Base density of resources (0.0 to 1.0)
    pub resource_density: f32,
    /// Scale of terrain features (higher = larger features)
    pub terrain_scale: f32,
    /// Water level (0.0 to 1.0)
    pub water_level: f32,
    /// Mountain level (0.0 to 1.0, must be > water_level)
    pub mountain_level: f32,
    /// Forest moisture threshold (0.0 to 1.0)
    pub forest_moisture: f32,
    /// Desert moisture threshold (0.0 to 1.0)
    pub desert_moisture: f32,
}

impl WorldConfig {
    /// Get the width of the world in tiles
    pub fn width_tiles(&self) -> u32 {
        (self.width_meters / self.tile_size) as u32
    }
    
    /// Get the height of the world in tiles
    pub fn height_tiles(&self) -> u32 {
        (self.height_meters / self.tile_size) as u32
    }
    
    /// Get the width of the playable area in meters (excluding borders)
    pub fn playable_width(&self) -> f32 {
        self.width_meters - 2.0 * self.border_width
    }
    
    /// Get the height of the playable area in meters (excluding borders)
    pub fn playable_height(&self) -> f32 {
        self.height_meters - 2.0 * self.border_width
    }
    
    /// Convert world position to tile coordinates
    pub fn world_to_tile(&self, pos: Vec2) -> (i32, i32) {
        (
            (pos.x / self.tile_size).floor() as i32,
            (pos.y / self.tile_size).floor() as i32
        )
    }
    
    /// Convert tile coordinates to world position (center of tile)
    pub fn tile_to_world(&self, x: i32, y: i32) -> Vec2 {
        Vec2::new(
            x as f32 * self.tile_size + self.tile_size / 2.0,
            y as f32 * self.tile_size + self.tile_size / 2.0
        )
    }
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            // Reduced from 10km to 1km for better performance
            width_meters: 1_000.0,  // 1km
            height_meters: 1_000.0, // 1km
            // Increased tile size from 10m to 20m to reduce total tile count
            tile_size: 20.0,         // 20m x 20m tiles
            // Reduced border width proportionally
            border_width: 50.0,     // 50m border
            seed: rand::random(),
            resource_density: 0.1,
            terrain_scale: 5.0,
            water_level: 0.3,
            mountain_level: 0.7,
            forest_moisture: 0.6,
            desert_moisture: 0.3,
        }
    }
}
