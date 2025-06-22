use bevy::prelude::*;
use crate::world::resource_types::{ResourceType, ResourceNode};

/// Component for resource node visuals
#[derive(Component)]
pub struct ResourceNodeVisual;

/// System to add sprites to resource nodes
pub fn add_resource_visuals(
    mut commands: Commands,
    query: Query<(Entity, &ResourceNode), Without<ResourceNodeVisual>>,
) {
    for (entity, resource_node) in &query {
        let resource_type = &resource_node.resource_type;
        let color = match resource_type {
            ResourceType::Minerals => Color::srgb(0.5, 0.5, 0.5),   // Gray
            ResourceType::Gas => Color::srgb(1.0, 0.65, 0.0),      // Orange
            ResourceType::Crystals => Color::srgb(0.8, 0.2, 0.8),  // Purple
            ResourceType::Energy => Color::srgb(1.0, 1.0, 0.0),    // Yellow
            ResourceType::Food => Color::srgb(0.0, 1.0, 0.0),      // Green
        };

        commands.entity(entity).insert((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(10.0)),
                    ..default()
                },
                ..default()
            },
            ResourceNodeVisual,
        ));
    }
}

/// Plugin for resource rendering
pub struct ResourceRenderPlugin;

impl Plugin for ResourceRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_resource_visuals);
    }
}
