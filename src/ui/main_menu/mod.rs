//! Main menu implementation for StrategyForge

mod components;
mod systems;

use bevy::prelude::*;
use crate::state::GameState;

/// Plugin for the main menu
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Run the UI camera setup when the app starts
            .add_systems(Startup, systems::setup_ui_camera)
            // Main menu systems
            .add_systems(OnEnter(GameState::MainMenu), systems::setup_main_menu)
            .add_systems(
                Update,
                systems::handle_menu_button_interactions
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(GameState::MainMenu), systems::cleanup_menu::<components::MainMenuMarker>);
    }
}
