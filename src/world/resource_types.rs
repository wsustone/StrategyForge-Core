use bevy::prelude::*;

/// Types of resources that can exist in the world
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ResourceType {
    Minerals,
    Gas,
    Crystals,
    Energy,
    Food,
}

/// Component representing a resource node in the world
#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct ResourceNode {
    /// Type of resource
    pub resource_type: ResourceType,
    /// Current amount of resources remaining
    pub amount: f32,
    /// Maximum capacity of this node
    pub max_amount: f32,
    /// Whether this resource can regenerate over time
    pub can_regenerate: bool,
    /// Current regeneration rate (units per second)
    pub regen_rate: f32,
}

impl ResourceNode {
    /// Creates a new ResourceNode with the specified properties
    pub fn new(resource_type: ResourceType, amount: f32, can_regenerate: bool) -> Self {
        Self {
            resource_type,
            amount,
            max_amount: amount,
            can_regenerate,
            regen_rate: if can_regenerate { 1.0 } else { 0.0 },
        }
    }
}

impl Default for ResourceNode {
    fn default() -> Self {
        Self {
            resource_type: ResourceType::Minerals,
            amount: 100.0,
            max_amount: 100.0,
            can_regenerate: false,
            regen_rate: 0.0,
        }
    }
}
