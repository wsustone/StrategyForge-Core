use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use crate::state::GameState;
use super::components::*;

// UI Constants
const WINDOW_BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PANEL_BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.35);
const BUTTON_SELECTED_COLOR: Color = Color::srgb(0.35, 0.35, 0.5);
const TEXT_COLOR: Color = Color::WHITE;
const TEXT_DISABLED_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.5);

// Helper function to create text style
fn regular_text_style(asset_server: &Res<AssetServer>, size: f32) -> TextStyle {
    TextStyle {
        font: asset_server.load("default"),
        font_size: size,
        color: TEXT_COLOR,
    }
}

// Settings resources
#[derive(Resource, Default)]
pub struct SettingsState {
    pub current_tab: u8,
    pub is_fullscreen: bool,
    pub vsync: bool,
    pub resolution: (u32, u32),
    pub has_unsaved_changes: bool,
}

// Helper function to create a setting row
fn create_setting_row(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    label: &str,
    control: SettingControl,
    _settings_state: &SettingsState,
) -> Entity {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    margin: UiRect::vertical(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            control,
        ))
        .with_children(|parent| {
            // Label
            parent.spawn(TextBundle::from_section(
                label,
                regular_text_style(asset_server, 20.0),
            ));
        })
        .id()
}
// Update the setup_settings_menu function to properly create tabs
pub(crate) fn setup_settings_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings_state: ResMut<SettingsState>,
) {
    // Spawn a 2D camera for the UI
    commands.spawn(Camera2dBundle::default());
    // Main container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: BackgroundColor(WINDOW_BG_COLOR),
                ..default()
            },
            SettingsMenuMarker,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn(TextBundle::from_section(
                "Settings",
                TextStyle {
                    font_size: 32.0,
                    color: TEXT_COLOR,
                    ..default()
                },
            ));

            // Tabs row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|tabs| {
                    // Add tab buttons
                    for (i, tab_name) in ["Video", "Audio", "Controls", "Gameplay"].iter().enumerate() {
                        let is_selected = settings_state.current_tab == i as u8;
                        tabs.spawn((
                            ButtonBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(10.0)),
                                    margin: UiRect::right(Val::Px(5.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(if is_selected {
                                    BUTTON_SELECTED_COLOR
                                } else {
                                    BUTTON_COLOR
                                }),
                                ..default()
                            },
                            TabButton::new(i as u8),
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                *tab_name,
                                TextStyle {
                                    font_size: 16.0,
                                    color: if is_selected { TEXT_COLOR } else { TEXT_DISABLED_COLOR },
                                    ..default()
                                },
                            ));
                        });
                    }
                });

            // Tab content area
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_grow: 1.0,
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    background_color: BackgroundColor(PANEL_BG_COLOR),
                    ..default()
                })
                .with_children(|content| {
                    // This will be populated based on the selected tab
                    match settings_state.current_tab {
                        0 => create_video_settings_tab(content, &asset_server, &settings_state),
                        _ => {}
                    }
                });

            // Bottom buttons row
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexEnd,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|buttons| {
                    // Back button
                    buttons
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(10.0)),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(BUTTON_COLOR),
                                ..default()
                            },
                            BackButton,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Back",
                                TextStyle {
                                    font_size: 16.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            ));
                        });

                    // Apply button
                    buttons
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(10.0)),
                                    margin: UiRect::right(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: BackgroundColor(BUTTON_COLOR),
                                ..default()
                            },
                            ApplyButton,
                        ))
                        .with_children(|button| {
                            button.spawn(TextBundle::from_section(
                                "Apply",
                                TextStyle {
                                    font_size: 16.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            ));
                        });

                    // Reset button
                    buttons.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(BUTTON_COLOR),
                            ..default()
                        },
                        ResetButton,
                    ))
                    .with_children(|button| {
                        button.spawn(TextBundle::from_section(
                            "Reset",
                            TextStyle {
                                font_size: 16.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        ));
                    });
                });
        });
}

// Helper function to create video settings tab
fn create_video_settings_tab(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    settings_state: &SettingsState,
) {
    parent
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|content| {
            // Add video settings controls here
            // Example: Resolution, Fullscreen, VSync, etc.
            create_setting_row(
                content,
                asset_server,
                "Fullscreen",
                SettingControl::new(SettingType::Video(VideoSettingControl::Fullscreen)),
                settings_state,
            );
        });
}



// Handle button interactions
pub(crate) fn handle_settings_button_interactions(
    interaction_query: Query<(&Interaction, Entity), (Changed<Interaction>, With<Button>)>,
    tab_button_query: Query<&TabButton>,
    back_button_query: Query<&BackButton>,
    apply_button_query: Query<&ApplyButton>,
    reset_button_query: Query<&ResetButton>,
    mut tab_content_query: Query<(&TabContent, &mut Visibility)>,
    mut settings_state: ResMut<SettingsState>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, entity) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            // Handle tab button clicks
            if let Ok(tab_button) = tab_button_query.get(entity) {
                settings_state.current_tab = tab_button.tab;
                
                // Update tab content visibility
                for (content, mut visibility) in tab_content_query.iter_mut() {
                    *visibility = if content.tab == tab_button.tab {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    };
                }
                continue;
            }

            // Handle back button
            if back_button_query.get(entity).is_ok() {
                game_state.set(GameState::MainMenu);
                continue;
            }

            // Handle apply button
            if apply_button_query.get(entity).is_ok() {
                // TODO: Apply settings
                settings_state.has_unsaved_changes = false;
                continue;
            }

            // Handle reset button
            if reset_button_query.get(entity).is_ok() {
                // TODO: Reset settings to defaults
                settings_state.has_unsaved_changes = true;
                continue;
            }
        }
    }
}

// Handle settings changes
pub(crate) fn handle_settings_changes(
    mut settings_state: ResMut<SettingsState>,
    mut interaction_query: Query<(&Interaction, &SettingControl), (Changed<Interaction>, With<Button>)>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for (interaction, control) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            if let Some(video_control) = control.as_video_setting_control() {
                match video_control {
                    VideoSettingControl::Fullscreen => {
                        if let Ok(mut window) = window_query.get_single_mut() {
                            settings_state.is_fullscreen = !settings_state.is_fullscreen;
                            window.mode = if settings_state.is_fullscreen {
                                WindowMode::Fullscreen
                            } else {
                                WindowMode::Windowed
                            };
                            settings_state.has_unsaved_changes = true;
                        }
                    }
                    VideoSettingControl::VSync => {
                        settings_state.vsync = !settings_state.vsync;
                        // TODO: Apply VSync setting to the renderer
                        settings_state.has_unsaved_changes = true;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub(crate) fn cleanup_menu<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

