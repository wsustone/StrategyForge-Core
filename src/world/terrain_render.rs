use bevy::prelude::*;
use super::{TerrainTile, TerrainType, ResourceNode};

/// Component for terrain tile visuals
#[derive(Component)]
pub struct TerrainTileVisual;

/// System to add sprites to terrain tiles
pub fn add_terrain_visuals(
    mut commands: Commands,
    query: Query<(Entity, &TerrainTile), (Without<TerrainTileVisual>, Without<ResourceNode>)>,
) {
    for (entity, tile) in &query {
        let color = match tile.terrain_type {
            TerrainType::Grassland => Color::srgb(0.2, 0.8, 0.2),    // Green
            TerrainType::Forest => Color::srgb(0.0, 0.6, 0.0),      // Dark Green
            TerrainType::Mountains => Color::srgb(0.5, 0.5, 0.5),   // Gray
            TerrainType::Desert => Color::srgb(0.93, 0.79, 0.69),   // Sand
            TerrainType::Water => Color::srgb(0.0, 0.4, 0.8),       // Blue
        };


        commands.entity(entity).insert((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(1.0)), // 1x1 unit size
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, -0.1), // Slightly behind other sprites
                ..default()
            },
            TerrainTileVisual,
        ));
    }
}

/// Plugin for terrain rendering
pub struct TerrainRenderPlugin;

impl Plugin for TerrainRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_terrain_visuals);
    }
}
