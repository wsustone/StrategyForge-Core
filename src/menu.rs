use bevy::prelude::*;

/// Main menu plugin that loads at game start
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_main_menu);
    }
}

fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Create menu container
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        Name::new("MainMenu"),
    ));

    // Menu items will be added by plugins
}

/// Trait for menu item plugins
pub trait MenuItemPlugin {
    fn add_menu_item(&self, app: &mut App, parent: Entity);
}
