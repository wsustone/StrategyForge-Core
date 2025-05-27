use bevy::prelude::*;
use bevy::ui::{AlignItems, JustifyContent, Style, Val};

/// Main menu plugin that loads at game start
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu);
    }
}

fn setup_main_menu(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    });
}

/// Trait for menu item plugins
pub trait MenuItemPlugin {
    fn add_menu_item(&self, app: &mut App, parent: Entity);
}
