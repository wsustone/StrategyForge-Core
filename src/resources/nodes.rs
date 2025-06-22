//! Resource node spawning and management

use bevy::prelude::*;
use crate::resources::{ResourceType, ResourceStorage, ResourceNode};

/// Component for entities that can gather resources
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component, Default)]
pub struct ResourceGatherer {
    /// Types of resources this unit can gather
    pub can_gather: Vec<ResourceType>,
    
    /// Current resource being carried (if any)
    pub carrying: Option<(ResourceType, f32)>,
    
    /// Maximum amount of resources that can be carried
    pub carry_capacity: f32,
    
    /// Gather rate (resources per second)
    pub gather_rate: f32,
    
    /// Distance at which the unit can gather resources
    pub gather_range: f32,
}

impl Default for ResourceGatherer {
    fn default() -> Self {
        Self {
            can_gather: vec![ResourceType::Wood, ResourceType::Stone],
            carrying: None,
            carry_capacity: 10.0,
            gather_rate: 1.0,
            gather_range: 2.0,
        }
    }
}

/// System to update resource nodes (regeneration, etc.)
pub fn update_resource_nodes(
    time: Res<Time>,
    mut query: Query<&mut ResourceNode>,
) {
    for mut node in query.iter_mut() {
        // Regenerate resources if applicable
        if node.can_regenerate && node.amount < node.max_amount {
            node.amount = (node.amount + node.regen_rate * time.delta_seconds())
                .min(node.max_amount);
        }
    }
}

/// System to handle resource gathering
pub fn handle_resource_gathering(
    time: Res<Time>,
    mut gatherers: Query<(&mut ResourceGatherer, &Transform)>,
    mut nodes: Query<(&mut ResourceNode, &Transform)>,
) {
    for (mut gatherer, gatherer_transform) in gatherers.iter_mut() {
        // Skip if already carrying maximum capacity
        if let Some((_, amount)) = gatherer.carrying {
            if amount >= gatherer.carry_capacity {
                continue;
            }
        }
        
        // Find the closest resource node of a type this gatherer can gather
        let gatherer_pos = gatherer_transform.translation;
        let mut best_node = None;
        let mut closest_distance = f32::MAX;
        
        for (node, node_transform) in nodes.iter_mut() {
            // Check if this node has resources and is of a type we can gather
            if node.amount <= 0.0 || !gatherer.can_gather.contains(&node.resource_type) {
                continue;
            }
            
            // Calculate distance to node
            let distance = node_transform.translation.distance_squared(gatherer_pos);
            if distance < closest_distance {
                closest_distance = distance;
                best_node = Some((node, node_transform));
            }
        }
        
        // If we found a node in range, gather from it
        if let Some((mut node, _)) = best_node {
            let gather_range_sq = gatherer.gather_range * gatherer.gather_range;
            if closest_distance <= gather_range_sq {
                // Calculate how much to gather this frame
                let gather_amount = gatherer.gather_rate * time.delta_seconds();
                let gathered = node.gather(gather_amount);
                
                // Add to carried resources
                if let Some((res_type, ref mut amount)) = &mut gatherer.carrying {
                    if *res_type == node.resource_type {
                        *amount += gathered;
                    }
                } else {
                    gatherer.carrying = Some((node.resource_type, gathered));
                }
            }
        }
    }
}

/// System to handle resource delivery to storage
pub fn handle_resource_delivery(
    mut gatherers: Query<(&mut ResourceGatherer, &Transform)>,
    mut storages: Query<(&mut ResourceStorage, &Transform)>,
    time: Res<Time>,
) {
    const DELIVERY_RANGE: f32 = 5.0; // Range at which resources can be delivered
    const DELIVERY_RATE: f32 = 10.0; // Resources per second
    
    for (mut gatherer, gatherer_transform) in gatherers.iter_mut() {
        // Skip if not carrying anything
        let (res_type, ref mut amount) = match &mut gatherer.carrying {
            Some(carrying) => carrying,
            None => continue,
        };
        
        // Find the closest storage that can accept this resource
        let gatherer_pos = gatherer_transform.translation;
        let mut best_storage = None;
        let mut closest_distance = f32::MAX;
        
        for (storage, storage_transform) in storages.iter_mut() {
            // Check if this storage can accept the resource
            if storage.get_remaining_capacity(*res_type) > 0 {
                let distance = storage_transform.translation.distance_squared(gatherer_pos);
                if distance < closest_distance {
                    closest_distance = distance;
                    best_storage = Some(storage);
                }
            }
        }
        
        // If we found a storage in range, deliver resources
        if let Some(mut storage) = best_storage {
            let delivery_range_sq = DELIVERY_RANGE * DELIVERY_RANGE;
            if closest_distance <= delivery_range_sq {
                // Calculate how much to deliver this frame
                let deliver_amount = (DELIVERY_RATE * time.delta_seconds()).min(*amount);
                
                // Try to add to storage
                let delivered = storage.add_resource(*res_type, deliver_amount as u32) as f32;
                *amount -= delivered;
                
                // If we delivered everything, clear the carried resource
                if *amount <= 0.0 {
                    gatherer.carrying = None;
                }
            }
        }
    }
}

/// Plugin for resource nodes and gathering
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<ResourceGatherer>()
            .add_systems(Update, (
                update_resource_nodes,
                handle_resource_gathering,
                handle_resource_delivery,
            ));
    }
}
