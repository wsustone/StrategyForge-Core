//! The core game engine and systems for StrategyForge.
//! This crate provides the foundation for all game plugins.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

use thiserror::Error;
extern crate libloading;

/// Game state management
/// 
/// This module handles the different states of the game (e.g., MainMenu, InGame, Settings)
/// and manages transitions between them.
pub mod state;

/// Main menu UI and interaction systems
/// 
/// This module contains the main menu UI components and systems for handling
/// user interactions with the main menu.
pub mod menu;

// Re-export commonly needed types
pub use bevy::prelude::*;
pub use state::GameState;

/// Re-export common Bevy types
pub use bevy::{
    app::{App, Plugin, PluginGroup, PluginGroupBuilder, Update, Startup, FixedUpdate},
    ecs::{
        component::Component,
        system::{Res, ResMut, Resource, Commands, Query},
        schedule::{IntoSystemConfigs, IntoSystemSetConfigs, ScheduleLabel, SystemSet},
        event::{Event, EventReader, EventWriter},
        query::With,
    },
    input::keyboard::KeyCode,
    input::mouse::MouseButton,
    input::ButtonInput,
    math::{Vec2, Vec3, Quat, Mat4},
    transform::components::Transform,
    window::PrimaryWindow,
    utils::default,
};

/// Error type for plugin loading
#[derive(Error, Debug)]
pub enum PluginLoadError {
    /// Failed to load the plugin library
    #[error("Failed to load plugin library: {0}")]
    LibraryLoad(#[from] libloading::Error),
    
    /// Failed to get plugin symbol
    #[error("Failed to get plugin symbol: {0}")]
    SymbolError(String),
    
    /// Plugin initialization failed
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),
}

/// Core plugin that sets up essential game systems
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Add the menu plugin
        app.add_plugins(menu::MenuPlugin);
        
        // Initialize the game state
        app.init_state::<state::GameState>();
    }
}

#[allow(dead_code)]
fn setup_core_systems() {
    // Core system initialization
    info!("Initializing StrategyForge Core");
}

/// Main plugin for StrategyForge Core
pub struct StrategyForgeCorePlugin;

impl Plugin for StrategyForgeCorePlugin {
    fn build(&self, app: &mut App) {
        // Setup core systems
        app.add_plugins(CorePlugin);
        
        // Setup plugin loading system
        // app.init_resource::<LoadedPlugins>(); // Dynamic plugin system removed
        // app.add_systems(Startup, load_initial_plugins); // Dynamic plugin system removed
    }
}

// /// Resource tracking loaded plugins (Dynamic plugin system removed)
// #[derive(Resource, Default)]
// pub struct LoadedPlugins {
//     plugins: Vec<Box<dyn crate::GamePlugin>>, // Assuming GamePlugin trait would be defined elsewhere or removed
// }

// /// System to load initial plugins (Dynamic plugin system removed)
// fn load_initial_plugins(
//     mut commands: Commands,
//     mut loaded_plugins: ResMut<LoadedPlugins>,
// ) {
//     // TODO: Load plugins from config
//     // For now we'll just log that plugin loading is ready
//     info!("Plugin system initialized - ready to load plugins");
// }


#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_core_plugin_initialization() {
        let mut app = App::new();
        app.add_plugins(CorePlugin);
        
        // Test that the plugin initializes without panicking
        app.update();
    }

    #[test]
    fn test_load_template_plugin() {
        let mut app = App::new();
        
        // Setup minimal required plugins
        app.add_plugins(MinimalPlugins);
        
        // Load the template plugin
        app.add_plugins(TemplatePlugin);
        
        // Verify plugin was loaded
        assert!(app.world.contains_resource::<LoadedPlugins>());
    }
}
