//! Resource storage and management

use bevy::prelude::*;
use std::collections::HashMap;
use super::ResourceType;

/// Component for entities that can store resources
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component, Default)]
pub struct ResourceStorage {
    capacities: HashMap<ResourceType, u32>,
    amounts: HashMap<ResourceType, u32>,
}

impl ResourceStorage {
    /// Create a new empty storage with default capacities
    pub fn new() -> Self {
        Self {
            capacities: HashMap::new(),
            amounts: HashMap::new(),
        }
    }
    
    /// Set the capacity for a specific resource type
    pub fn set_capacity(&mut self, resource_type: ResourceType, capacity: u32) {
        self.capacities.insert(resource_type, capacity);
        // Ensure amount doesn't exceed new capacity
        if let Some(amount) = self.amounts.get_mut(&resource_type) {
            *amount = (*amount).min(capacity);
        }
    }
    
    /// Get the current amount of a resource
    pub fn get_amount(&self, resource_type: ResourceType) -> u32 {
        *self.amounts.get(&resource_type).unwrap_or(&0)
    }
    
    /// Get the capacity for a resource type
    pub fn get_capacity(&self, resource_type: ResourceType) -> u32 {
        *self.capacities.get(&resource_type).unwrap_or(&0)
    }
    
    /// Get the remaining capacity for a resource type
    pub fn get_remaining_capacity(&self, resource_type: ResourceType) -> u32 {
        self.get_capacity(resource_type).saturating_sub(self.get_amount(resource_type))
    }
    
    /// Add resources of a specific type
    /// Returns the amount that was actually added
    pub fn add_resource(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let capacity = self.get_capacity(resource_type);
        let current = self.get_amount(resource_type);
        let add_amount = amount.min(capacity.saturating_sub(current));
        
        if add_amount > 0 {
            *self.amounts.entry(resource_type).or_insert(0) += add_amount;
        }
        
        add_amount
    }
    
    /// Remove resources of a specific type
    /// Returns the amount that was actually removed
    pub fn remove_resource(&mut self, resource_type: ResourceType, amount: u32) -> u32 {
        let current = self.get_amount(resource_type);
        let remove_amount = amount.min(current);
        
        if remove_amount > 0 {
            *self.amounts.get_mut(&resource_type).unwrap() -= remove_amount;
        }
        
        remove_amount
    }
    
    /// Check if there's enough of a resource
    pub fn has_enough(&self, resource_type: ResourceType, amount: u32) -> bool {
        self.get_amount(resource_type) >= amount
    }
    
    /// Check if the storage has any resources
    pub fn is_empty(&self) -> bool {
        self.amounts.values().all(|&amount| amount == 0)
    }
    
    /// Get an iterator over all stored resources
    pub fn iter(&self) -> impl Iterator<Item = (ResourceType, u32, u32)> + '_ {
        self.amounts.iter().map(move |(&resource_type, &amount)| {
            let capacity = self.get_capacity(resource_type);
            (resource_type, amount, capacity)
        })
    }
}
