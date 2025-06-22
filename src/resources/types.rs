//! Resource type definitions for StrategyForge

use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fmt;

/// Primary resource types in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Reflect, Default)]
#[reflect(Debug, PartialEq, Hash)]
pub enum ResourceType {
    #[default]
    /// Basic building material, gathered from trees and forests
    Wood,
    
    /// Stone used for defensive structures and upgrades
    Stone,
    
    /// Iron used for advanced units and structures
    Iron,
    
    /// Copper used for advanced units and structures
    Copper,
    
    /// Alloy created from primary resources
    Alloy,
    
    /// Refined materials for advanced construction
    Refined,
}

impl ResourceType {
    /// Get the default storage capacity for this resource type
    pub fn default_capacity(&self) -> u32 {
        match self {
            ResourceType::Wood => 1000,
            ResourceType::Stone => 500,
            ResourceType::Iron => 200,
            ResourceType::Copper => 200,
            ResourceType::Alloy => 100,
            ResourceType::Refined => 50,
        }
    }
    
    /// Get the base gather rate for this resource type (per second)
    pub fn base_gather_rate(&self) -> f32 {
        match self {
            ResourceType::Wood => 5.0,
            ResourceType::Stone => 3.0,
            ResourceType::Iron => 2.0,
            ResourceType::Copper => 2.0,
            ResourceType::Alloy => 1.0,
            ResourceType::Refined => 0.5,
        }
    }
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResourceType::Wood => write!(f, "Wood"),
            ResourceType::Stone => write!(f, "Stone"),
            ResourceType::Iron => write!(f, "Iron"),
            ResourceType::Copper => write!(f, "Copper"),
            ResourceType::Alloy => write!(f, "Alloy"),
            ResourceType::Refined => write!(f, "Refined"),
        }
    }
}

/// Component representing a resource node in the world
#[derive(Component, Debug, Clone, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ResourceNode {
    /// Type of resource this node provides
    pub resource_type: ResourceType,
    
    /// Current amount of resources remaining
    pub amount: f32,
    
    /// Maximum capacity of this node
    pub max_amount: f32,
    
    /// Whether this node can regenerate over time
    pub can_regenerate: bool,
    
    /// Rate at which the node regenerates (per second)
    pub regen_rate: f32,
}

impl ResourceNode {
    /// Create a new resource node
    pub fn new(resource_type: ResourceType, amount: f32, can_regenerate: bool) -> Self {
        let max_amount = amount;
        Self {
            resource_type,
            amount,
            max_amount,
            can_regenerate,
            regen_rate: match resource_type {
                ResourceType::Wood => 0.1,  // Trees regrow slowly
                _ => 0.0,  // Other resources don't regenerate by default
            },
        }
    }
    
    /// Extract resources from the node
    /// Returns the amount actually gathered
    pub fn gather(&mut self, amount: f32) -> f32 {
        let gathered = amount.min(self.amount);
        self.amount -= gathered;
        gathered
    }
    
    /// Check if the node is depleted
    pub fn is_depleted(&self) -> bool {
        self.amount <= 0.0
    }
}
