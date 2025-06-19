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
use crate::ui::theme::ButtonTheme;

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
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        padding: UiRect::all(Val::Px(10.0)),
                                        margin: UiRect::top(Val::Px(30.0)),
                                        ..Default::default()
                                    },
                                    background_color: Color::NONE.into(),
                                    border_color: theme.ui.error.into(),
                                    ..Default::default()
                                },
                                ButtonHoverEffect {
                                    normal: Color::NONE,
                                    hovered: Color::srgba(1.0, 0.2, 0.2, 0.2),
                                    pressed: Color::srgba(1.0, 0.1, 0.1, 0.3),
                                },
                                ExitButton,
                            )).with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    "EXIT",
                                    TextStyle {
                                        font: default(), // Use Bevy's default font
                                        font_size: 24.0,
                                        color: theme.ui.error,
                                    },
                                ));
                            });
                        });
                });
        });
}

fn spawn_menu_button<B: Component + Clone>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    theme: &Theme,
    text: &str,
    button_type: B,
) -> Entity {
    // Create a custom button theme for menu buttons
    let button_theme = ButtonTheme {
        normal: theme.button.normal,
        hovered: theme.button.hovered,
        pressed: theme.button.pressed,
        disabled: theme.button.disabled,
        text: theme.button.text,
        border: theme.button.border,
        border_thickness: theme.button.border_thickness,
        corner_radius: theme.button.corner_radius,
    };
    
    // Create text style with default font
    let text_style = TextStyle {
        font: default(), // Use Bevy's default font
        font_size: 24.0,
        color: theme.ui.text,
    };
    
    // Spawn the button with text and button type
    let mut commands = parent.spawn((
        ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..Default::default()
            },
            background_color: button_theme.normal.into(),
            ..Default::default()
        },
        button_type,
    ));
    
    // Add hover effect
    commands.insert(ButtonHoverEffect {
        normal: button_theme.normal,
        hovered: button_theme.hovered,
        pressed: button_theme.pressed,
    });
    
    // Add text as a child
    commands.with_children(|parent| {
        parent.spawn(TextBundle::from_section(text, text_style));
    });
    
    commands.id()
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
