//! Settings menu UI components and systems

// Module declarations
pub mod components;
pub mod styles;
mod systems;  // Don't export systems directly

use bevy::prelude::*;
use crate::state::GameState;

// Re-export only what's needed
pub use components::{
    SettingsState, SettingsTab, SettingsMenuMarker, SettingType, VideoSettingControl,
    VideoSettings, SettingControl, TabButton, TabContent, ApplyButton, ResetButton, BackButton,
    SettingsChangedEvent, ApplySettingsEvent, ResetSettingsEvent, BackToMenuEvent,
    DisplayMode, GraphicsQuality
};
pub use styles::*;

/// Setup function to be called by the main app's plugin
pub fn setup_settings_ui(app: &mut App) {
    app
        .insert_resource(SettingsState {
            current_tab: 0,
            video_settings: VideoSettings {
                display_mode: DisplayMode::Windowed,
                resolution: (1920, 1080),
                graphics_quality: GraphicsQuality::High,
                brightness: 1.0,
                contrast: 1.0,
                vsync: true,
                fps_limit: Some(60),
                ui_scale: 1.0,
            },
        })
        .add_systems(OnEnter(GameState::Settings), systems::setup_settings_menu)
        .add_systems(
            Update,
            (
                systems::handle_settings_button_interactions,
                systems::handle_settings_changes,
            )
            .run_if(in_state(GameState::Settings)),
        )
        .add_systems(OnExit(GameState::Settings), systems::cleanup_menu::<SettingsMenuMarker>);
}
