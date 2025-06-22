use bevy::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use super::{WorldConfig, GameWorld, TerrainType, TerrainTile};
use super::resource_types::{ResourceType, ResourceNode};

/// Component marking a resource node in the world
#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct ResourceNodeMarker;

/// Configuration for resource spawning
#[derive(Debug, Clone)]
pub struct ResourceSpawnConfig {
    /// Type of resource to spawn
    pub resource_type: ResourceType,
    /// Base amount of resources in each node
    pub base_amount: f32,
    /// Whether the resource can regenerate
    pub can_regenerate: bool,
    /// Biomes where this resource can spawn
    pub spawn_biomes: Vec<TerrainType>,
    /// Density of this resource (0.0 to 1.0)
    pub density: f32,
    /// Variation in amount (0.0 to 1.0)
    pub amount_variation: f32,
}

impl Default for ResourceSpawnConfig {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::Minerals, // Using the default resource type
            base_amount: 50.0,
            can_regenerate: true,
            spawn_biomes: vec![TerrainType::Forest, TerrainType::Grassland],
            density: 0.1,
            amount_variation: 0.3,
        }
    }
}

/// System to spawn initial resources in the world
pub fn spawn_initial_resources(
    mut commands: Commands,
    world_query: Query<Entity, With<GameWorld>>,
    config: Res<WorldConfig>,
    terrain_query: Query<(&TerrainTile, &Transform), Without<ResourceNodeMarker>>,
) {
    if let Ok(world_entity) = world_query.get_single() {
        // Define resource spawn configurations based on world config
        let resource_configs = vec![
            ResourceSpawnConfig {
                resource_type: ResourceType::Minerals, // Using Minerals instead of Wood
                base_amount: 100.0,
                can_regenerate: true,
                spawn_biomes: vec![TerrainType::Forest, TerrainType::Grassland],
                density: 0.15 * config.resource_density * 10.0, // Scale density based on config
                amount_variation: 0.5,
            },
            ResourceSpawnConfig {
                resource_type: ResourceType::Minerals, // Using Minerals instead of Stone
                base_amount: 50.0,
                can_regenerate: false,
                spawn_biomes: vec![TerrainType::Mountains],
                density: 0.1 * config.resource_density * 10.0, // Scale density based on config
                amount_variation: 0.4,
            },
            ResourceSpawnConfig {
                resource_type: ResourceType::Crystals, // Using Crystals instead of Iron
                base_amount: 25.0,
                can_regenerate: false,
                spawn_biomes: vec![TerrainType::Mountains],
                density: 0.05 * config.resource_density * 10.0, // Scale density based on config
                amount_variation: 0.3,
            },
        ];

        // Spawn each resource type according to its configuration
        for config in resource_configs {
            // Collect valid positions for this resource type
            let mut valid_positions = Vec::new();
            
            // Find all valid positions for this resource type
            for (terrain_tile, transform) in &terrain_query {
                if config.spawn_biomes.contains(&terrain_tile.terrain_type) {
                    valid_positions.push(transform.translation.truncate());
                }
            }
            
            // Shuffle the positions to get random distribution
            let mut rng = rand::thread_rng();
            valid_positions.shuffle(&mut rng);
            
            // Calculate how many resources to spawn based on density
            let num_resources = (valid_positions.len() as f32 * config.density) as usize;
            
            // Spawn resources at the selected positions
            for pos in valid_positions.into_iter().take(num_resources) {
                // Calculate amount with variation
                let variation = 1.0 + (rng.gen::<f32>() * 2.0 - 1.0) * config.amount_variation;
                let amount = (config.base_amount * variation).max(1.0);
                
                spawn_resource_node(
                    &mut commands,
                    world_entity,
                    config.resource_type,
                    pos,
                    amount,
                    config.can_regenerate,
                );
            }
        }
        
        // Add copper to the resource configurations
        let copper_config = ResourceSpawnConfig {
            resource_type: ResourceType::Gas, // Using Gas instead of Copper
            base_amount: 60.0,
            can_regenerate: false,
            spawn_biomes: vec![TerrainType::Mountains],
            density: 0.02,  // Rarer than other resources
            amount_variation: 0.4,
        };
        
        // Spawn copper nodes
        let mut valid_positions = Vec::new();
        for (terrain_tile, transform) in &terrain_query {
            if copper_config.spawn_biomes.contains(&terrain_tile.terrain_type) {
                valid_positions.push(transform.translation.truncate());
            }
        }
        
        // Shuffle and take the required number of positions
        let mut rng = rand::thread_rng();
        valid_positions.shuffle(&mut rng);
        let num_resources = (valid_positions.len() as f32 * copper_config.density) as usize;
        
        for pos in valid_positions.into_iter().take(num_resources) {
            let variation = 1.0 + (rng.gen::<f32>() * 2.0 - 1.0) * copper_config.amount_variation;
            let amount = (copper_config.base_amount * variation).max(1.0);
            
            spawn_resource_node(
                &mut commands,
                world_entity,
                copper_config.resource_type,
                pos,
                amount,
                copper_config.can_regenerate,
            );
        }
    }
}

/// Spawns a single resource node in the world at the specified position
fn spawn_resource_node(
    commands: &mut Commands,
    world_entity: Entity,
    resource_type: ResourceType,
    position: Vec2,
    amount: f32,
    can_regenerate: bool,
) -> Entity {
    let node_entity = commands.spawn((
        ResourceNode::new(resource_type, amount, can_regenerate),
        ResourceNodeMarker,
        Transform::from_xyz(position.x, position.y, 1.0),
        GlobalTransform::default(),
    )).id();
    
    // Make the resource node a child of the world
    commands.entity(world_entity).add_child(node_entity);
    
    node_entity
}
