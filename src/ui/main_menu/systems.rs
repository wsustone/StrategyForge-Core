//! Systems for the main menu

use bevy::{
    prelude::*,
    app::AppExit,
    ui::{
        Val, UiRect, Style, FlexDirection, JustifyContent, AlignItems, 
    },
};
use crate::state::GameState;
use crate::ui::components::ButtonHoverEffect;
use crate::ui::theme::Theme;
use crate::ui::utils::create_button;

// Import button components from the main menu components module
use super::components::{
    NewGameButton, LoadGameButton, MultiplayerButton, 
    SettingsButton, CreditsButton, ExitButton, MainMenuMarker
};

pub fn setup_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    theme: Res<Theme>,
) {
    // Spawn a 2D camera for the UI
    commands.spawn(Camera2dBundle::default());
    
    // Use Bevy's default font
    let font = asset_server.load("default");
    let regular_font = font.clone();
    
    // Root node with background
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: BackgroundColor(theme.ui.background),
                ..default()
            },
            MainMenuMarker,
        ))
        .with_children(|parent| {
            // Main container with max width for better readability on wide screens
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(20.0)),
                            max_width: Val::Px(800.0),
                            ..default()
                        },
                        ..default()
                    },
                    MainMenuMarker,
                ))
                .with_children(|parent| {
                    // Game title
                    parent.spawn(TextBundle::from_section(
                        "STRATEGYFORGE",
                        TextStyle {
                            font: font.clone(),
                            font_size: 72.0,
                            color: theme.ui.accent,
                        },
                    ));

                    // Version text (smaller, below title)
                    parent.spawn(TextBundle::from_section(
                        format!("v{}", env!("CARGO_PKG_VERSION")),
                        TextStyle {
                            font: regular_font.clone(),
                            font_size: 24.0,
                            color: theme.ui.text_secondary,
                        },
                    ));

                    // Spacer between title and buttons
                    parent.spawn(NodeBundle {
                        style: Style {
                            min_width: Val::Auto,
                            min_height: Val::Px(40.0),
                            ..default()
                        },
                        ..default()
                    });

                    // Menu buttons container
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Stretch,
                                    width: Val::Px(300.0),
                                    row_gap: Val::Px(15.0),
                                    column_gap: Val::Px(0.0),
                                    ..default()
                                },
                                ..default()
                            },
                            MainMenuMarker,
                        ))
                        .with_children(|parent| {
                            // New Game button
                            spawn_menu_button(
                                parent,
                                &asset_server,
                                &theme,
                                "NEW GAME",
                                NewGameButton,
                            );

                            // Load Game button
                            spawn_menu_button(
                                parent,
                                &asset_server,
                                &theme,
                                "LOAD GAME",
                                LoadGameButton,
                            );

                            // Multiplayer button
                            spawn_menu_button(
                                parent,
                                &asset_server,
                                &theme,
                                "MULTIPLAYER",
                                MultiplayerButton,
                            );

                            // Settings button
                            spawn_menu_button(
                                parent,
                                &asset_server,
                                &theme,
                                "SETTINGS",
                                SettingsButton,
                            );

                            // Credits button
                            spawn_menu_button(
                                parent,
                                &asset_server,
                                &theme,
                                "CREDITS",
                                CreditsButton,
                            );

                            // Exit button with different styling
                            let (button_bundle, hover_effect) = create_button(
                                &theme.button,
                                "EXIT",
                                font.clone(),
                            );
                            
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            margin: UiRect::top(Val::Px(30.0)),
                                            ..button_bundle.style
                                        },
                                        background_color: Color::NONE.into(),
                                        border_color: theme.ui.error.into(),
                                        ..button_bundle
                                    },
                                    hover_effect,
                                    ExitButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "EXIT",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 24.0,
                                            color: theme.ui.error,
                                        },
                                    ));
                                });
                        });
                });
        });
}

fn spawn_menu_button<B: Component>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    theme: &Theme,
    text: &str,
    button_type: B,
) -> Entity {
    // Create a simple button style directly instead of using create_button
    let button_style = Style {
        width: Val::Percent(100.0),
        height: Val::Px(50.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        margin: UiRect::bottom(Val::Px(10.0)),
        ..default()
    };
    
    let normal_color = Color::srgb(0.15, 0.15, 0.2);
    let hover_color = Color::srgb(0.2, 0.2, 0.25);
    let press_color = Color::srgb(0.1, 0.1, 0.15);
    
    parent
        .spawn((
            ButtonBundle {
                style: button_style,
                background_color: BackgroundColor(normal_color),
                ..default()
            },
            ButtonHoverEffect {
                normal: normal_color,
                hovered: hover_color,
                pressed: press_color,
            },
            button_type,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: theme.ui.text,
                },
            ));
        })
        .id()
}

pub fn handle_menu_button_interactions(
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &ButtonHoverEffect,
        Option<&NewGameButton>,
        Option<&LoadGameButton>,
        Option<&MultiplayerButton>,
        Option<&SettingsButton>,
        Option<&CreditsButton>,
        Option<&ExitButton>,
    ), (Changed<Interaction>, With<Button>)>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut bg_color, effect, new_game, load_game, multiplayer, settings, credits, exit) in interaction_query.iter_mut() {
        // Update button appearance based on interaction state
        *bg_color = match interaction {
            Interaction::Pressed => BackgroundColor(effect.pressed),
            Interaction::Hovered => BackgroundColor(effect.hovered),
            Interaction::None => BackgroundColor(effect.normal),
        };

        // Only process button press events
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Handle button actions based on button type
        if new_game.is_some() {
            next_state.set(GameState::Singleplayer);
        } else if load_game.is_some() {
            // TODO: Implement load game
            println!("Load game clicked");
        } else if multiplayer.is_some() {
            // TODO: Implement multiplayer
            println!("Multiplayer clicked");
        } else if settings.is_some() {
            next_state.set(GameState::Settings);
        } else if credits.is_some() {
            // TODO: Show credits
            println!("Credits clicked");
        } else if exit.is_some() {
            app_exit_events.send(AppExit::Success);
        }
    }
}

pub fn cleanup_menu<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
