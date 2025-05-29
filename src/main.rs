use bevy::prelude::*;
use bevy::app::AppExit;
use strategyforge_core::StrategyForgeCorePlugin;
use strategyforge_core::menu::{ExitButton, register_menu_plugin};
use sf_settings_menu::SettingsMenuPlugin;
use sf_campaign_menu::CampaignMenuPlugin;

fn main() {
    // Register menu plugins before app creation
    register_menu_plugin(Box::new(SettingsMenuPlugin::default()));
    register_menu_plugin(Box::new(CampaignMenuPlugin::default()));
    
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StrategyForgeCorePlugin) // This already includes the MenuPlugin
        .add_systems(Startup, setup_camera)
        .add_systems(Update, handle_exit_button)
        .run();
}

/// Set up a camera for UI rendering
fn setup_camera(mut commands: Commands) {
    // Create a 2D camera for UI
    commands.spawn(Camera2dBundle::default());
}

// Using the ExitButton component from the menu module



/// System to handle the exit button click
fn handle_exit_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ExitButton>)>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Exit the application when the button is pressed
                app_exit_events.send(AppExit::Success);
            }
            Interaction::Hovered => {
                // Change color when hovered
                *color = Color::srgb(0.6, 0.3, 0.3).into();
            }
            Interaction::None => {
                // Reset color when not interacting
                *color = Color::srgb(0.5, 0.2, 0.2).into();
            }
        }
    }
}
