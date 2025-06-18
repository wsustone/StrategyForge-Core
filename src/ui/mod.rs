//! UI system for StrategyForge
//! 
//! This module handles all user interface elements including menus, HUD, and in-game UI.

mod components;
mod styles;
mod theme;
mod utils;

pub mod main_menu;
pub mod settings;
mod campaign;

pub use components::*;
pub use styles::*;
pub use theme::*;
pub use utils::*;

pub use main_menu::MainMenuPlugin;
pub use campaign::CampaignMenuPlugin;

use bevy::prelude::*;
use crate::state::GameState;

// Re-export commonly used UI components and styles
// These are used by other modules that import from this module

/// Plugin that sets up all UI components and systems
/// Plugin that sets up all UI systems
pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // Register components for reflection
        app.register_type::<ButtonHoverEffect>()
            .register_type::<Panel>()
            .register_type::<WindowPanel>()
            .register_type::<TabButton>()
            .register_type::<Tooltip>();
        
        // Add theme plugin
        app.add_plugins(ThemePlugin);
        
        // Add UI systems
        app.add_systems(
            Update,
            (
                button_effect_system,
                update_tab_buttons,
                update_tooltips,
            ).run_if(in_state(GameState::MainMenu).or_else(in_state(GameState::Settings))),
        );
        
        // Add menu plugins
        app.add_plugins((
            MainMenuPlugin,
            CampaignMenuPlugin,
        ));
        
        // Setup settings UI
        settings::setup_settings_ui(app);
    }
}

/// System to update tab button states
fn update_tab_buttons(
    mut interaction_query: Query<(&Interaction, &mut TabButton, &mut BackgroundColor), Changed<Interaction>>,
    theme: Res<Theme>,
) {
    for (interaction, tab_button, mut color) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            *color = if tab_button.selected {
                theme.tab.selected.into()
            } else {
                theme.tab.hovered.into()
            };
        }
    }
}

/// System to update tooltip positions
fn update_tooltips(
    mut tooltip_query: Query<(&mut Style, &Tooltip, &GlobalTransform), With<Tooltip>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window_query.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    
    for (mut style, tooltip, _transform) in tooltip_query.iter_mut() {
        if let Some(position) = camera.viewport_to_world_2d(
            camera_transform,
            window.cursor_position().unwrap_or_default(),
        ) {
            style.left = Val::Px(position.x + tooltip.offset.x);
            style.top = Val::Px(position.y + tooltip.offset.y);
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}
