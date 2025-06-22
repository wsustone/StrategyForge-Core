//! Resource system for StrategyForge
//! Handles all resource-related functionality including resource types, storage, and gathering

use bevy::prelude::*;

mod nodes;
mod storage;
pub mod types;

pub use nodes::*;
pub use storage::*;
pub use types::*;

/// Plugin for the resource system
pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(&self, app: &mut App) {
        // Register all resource-related types for reflection
        app
            .register_type::<ResourceType>()
            .register_type::<ResourceNode>()
            .register_type::<ResourceStorage>()
            .register_type::<ResourceGatherer>();
        
        // Add resource systems
        app.add_systems(Update, (
            update_resource_nodes,
            handle_resource_gathering,
            handle_resource_delivery,
        ));
    }
}

// Re-export the ResourceNode component for use in other modules
pub use types::ResourceNode;
