use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use crate::state::GameState;
use super::{
    SettingsState, SettingControl, SettingType, VideoSettingControl,
    TabButton, BackButton, ApplyButton, ResetButton, TabContent, SettingsMenuMarker
};
use super::components::{DisplayMode, GraphicsQuality};
use bevy::ui::BackgroundColor;
use bevy::window::WindowResolution;

// UI Constants
const WINDOW_BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.1);
const PANEL_BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.35);
const BUTTON_SELECTED_COLOR: Color = Color::srgb(0.35, 0.35, 0.5);
const TEXT_COLOR: Color = Color::WHITE;
const TEXT_DISABLED_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.5);

// Helper function to create text style
fn regular_text_style(_asset_server: &Res<AssetServer>, size: f32) -> TextStyle {
    TextStyle {
        font: default(), // Use Bevy's default font
        font_size: size,
        color: TEXT_COLOR,
    }
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
            // Header section with title and tabs
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        width: Val::Percent(100.0),
                        margin: UiRect::bottom(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|header| {
                    // Title with bottom margin
                    header.spawn(TextBundle::from_section(
                        "Settings",
                        TextStyle {
                            font_size: 40.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    ));

                    // Tabs row with subtle bottom border
                    header
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::FlexEnd,
                                margin: UiRect::top(Val::Px(20.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|tabs| {
                            // Add tab buttons with better styling
                            for (i, tab_name) in ["Video", "Audio", "Controls", "Gameplay"].iter().enumerate() {
                                let is_selected = settings_state.current_tab == i as u8;
                                tabs.spawn((
                                    ButtonBundle {
                                        style: Style {
                                            padding: UiRect::horizontal(Val::Px(20.0)).with_top(Val::Px(10.0)),
                                            margin: UiRect::right(Val::Px(2.0)),
                                            ..default()
                                        },
                                        background_color: BackgroundColor(if is_selected {
                                            PANEL_BG_COLOR
                                        } else {
                                            PANEL_BG_COLOR.with_alpha(0.5)
                                        }),
                                        ..default()
                                    },
                                    TabButton::new(i as u8),
                                ))
                                .with_children(|button| {
                                    button.spawn(TextBundle::from_section(
                                        *tab_name,
                                        TextStyle {
                                            font_size: 18.0,
                                            color: if is_selected { 
                                                Color::srgb(0.9, 0.9, 1.0) 
                                            } else { 
                                                TEXT_DISABLED_COLOR 
                                            },
                                            ..default()
                                        },
                                    ));
                                });
                            }
                        });
                });

            // Main content area with tab content and action buttons
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        flex_grow: 1.0,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|content| {
                    // Tab content area with subtle border
                    content
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                padding: UiRect::all(Val::Px(20.0)),
                                margin: UiRect::bottom(Val::Px(20.0)),
                                ..default()
                            },
                            background_color: BackgroundColor(PANEL_BG_COLOR),
                            ..default()
                        })
                        .with_children(|tab_content| {
                            // This will be populated based on the selected tab
                            match settings_state.current_tab {
                                0 => create_video_settings_tab(tab_content, &asset_server, &settings_state),
                                _ => {}
                            }
                        });

                    // Bottom action buttons with proper spacing
                    content
                        .spawn(NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|buttons| {
                            // Back button on the left
                            buttons
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            padding: UiRect::axes(Val::Px(30.0), Val::Px(12.0)),
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
                                            font_size: 18.0,
                                            color: TEXT_COLOR,
                                            ..default()
                                        },
                                    ));
                                });

                            // Action buttons on the right
                            buttons
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|action_buttons| {
                                    // Reset button
                                    action_buttons
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
                                                    margin: UiRect::right(Val::Px(15.0)),
                                                    ..default()
                                                },
                                                background_color: BackgroundColor(BUTTON_COLOR),
                                                ..default()
                                            },
                                            ResetButton,
                                        ))
                                        .with_children(|button| {
                                            button.spawn(TextBundle::from_section(
                                                "Reset to Defaults",
                                                TextStyle {
                                                    font_size: 16.0,
                                                    color: TEXT_COLOR,
                                                    ..default()
                                                },
                                            ));
                                        });

                                    // Apply button with accent color
                                    action_buttons
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    padding: UiRect::axes(Val::Px(30.0), Val::Px(12.0)),
                                                    ..default()
                                                },
                                                background_color: BackgroundColor(Color::srgb(0.2, 0.5, 0.8)),
                                                ..default()
                                            },
                                            ApplyButton,
                                        ))
                                        .with_children(|button| {
                                            button.spawn(TextBundle::from_section(
                                                "Apply",
                                                TextStyle {
                                                    font_size: 18.0,
                                                    color: Color::WHITE,
                                                    ..default()
                                                },
                                            ));
                                        });
                                });
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
                width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|content| {
            // Display Mode
            create_setting_row(
                content,
                asset_server,
                "Display Mode",
                SettingControl::new(SettingType::Video(VideoSettingControl::DisplayMode)),
                settings_state,
            );

            // Resolution
            create_setting_row(
                content,
                asset_server,
                "Resolution",
                SettingControl::new(SettingType::Video(VideoSettingControl::Resolution)),
                settings_state,
            );

            // Graphics Quality
            create_setting_row(
                content,
                asset_server,
                "Graphics Quality",
                SettingControl::new(SettingType::Video(VideoSettingControl::GraphicsQuality)),
                settings_state,
            );

            // Brightness
            create_setting_row(
                content,
                asset_server,
                "Brightness",
                SettingControl::new(SettingType::Video(VideoSettingControl::Brightness)),
                settings_state,
            );

            // Contrast
            create_setting_row(
                content,
                asset_server,
                "Contrast",
                SettingControl::new(SettingType::Video(VideoSettingControl::Contrast)),
                settings_state,
            );

            // VSync
            create_setting_row(
                content,
                asset_server,
                "VSync",
                SettingControl::new(SettingType::Video(VideoSettingControl::VSync)),
                settings_state,
            );

            // FPS Limit
            create_setting_row(
                content,
                asset_server,
                "FPS Limit",
                SettingControl::new(SettingType::Video(VideoSettingControl::FpsLimit)),
                settings_state,
            );

            // UI Scale
            create_setting_row(
                content,
                asset_server,
                "UI Scale",
                SettingControl::new(SettingType::Video(VideoSettingControl::UiScale)),
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
                info!("Settings applied");
                continue;
            }

            // Handle reset button
            if reset_button_query.get(entity).is_ok() {
                // TODO: Reset settings to defaults
                info!("Settings reset to defaults");
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
                if let Ok(mut window) = window_query.get_single_mut() {
                    match video_control {
                        VideoSettingControl::DisplayMode => {
                            // Cycle through display modes: Windowed -> Borderless -> Fullscreen -> Windowed
                            settings_state.video_settings.display_mode = match settings_state.video_settings.display_mode {
                                DisplayMode::Windowed => DisplayMode::Borderless,
                                DisplayMode::Borderless => DisplayMode::Fullscreen,
                                DisplayMode::Fullscreen => DisplayMode::Windowed,
                            };
                            
                            // Apply the display mode to the window
                            window.mode = match settings_state.video_settings.display_mode {
                                DisplayMode::Windowed => WindowMode::Windowed,
                                DisplayMode::Borderless => WindowMode::BorderlessFullscreen,
                                DisplayMode::Fullscreen => WindowMode::Fullscreen,
                            };
                        }
                        VideoSettingControl::Resolution => {
                            // Cycle through common resolutions
                            let resolutions = [
                                (1280, 720),
                                (1366, 768),
                                (1600, 900),
                                (1920, 1080),
                                (2560, 1440),
                                (3840, 2160),
                            ];
                            
                            if let Some(pos) = resolutions.iter().position(|&r| r == settings_state.video_settings.resolution) {
                                let next_pos = (pos + 1) % resolutions.len();
                                settings_state.video_settings.resolution = resolutions[next_pos];
                            } else {
                                settings_state.video_settings.resolution = (1920, 1080); // Default to 1080p
                            }
                            
                            // Apply resolution if in windowed mode
                            if let WindowMode::Windowed = window.mode {
                                window.resolution = WindowResolution::new(
                                settings_state.video_settings.resolution.0 as f32,
                                settings_state.video_settings.resolution.1 as f32
                            );
                            }
                        }
                        VideoSettingControl::GraphicsQuality => {
                            // Cycle through quality presets
                            settings_state.video_settings.graphics_quality = match settings_state.video_settings.graphics_quality {
                                GraphicsQuality::Low => GraphicsQuality::Medium,
                                GraphicsQuality::Medium => GraphicsQuality::High,
                                GraphicsQuality::High => GraphicsQuality::Ultra,
                                GraphicsQuality::Ultra => GraphicsQuality::Custom,
                                GraphicsQuality::Custom => GraphicsQuality::Low,
                            };
                            // TODO: Apply graphics quality settings
                        }
                        VideoSettingControl::Brightness => {
                            // Cycle brightness between 0.5 and 1.5 in 0.1 increments
                            settings_state.video_settings.brightness = (settings_state.video_settings.brightness + 0.1) % 1.6;
                            if settings_state.video_settings.brightness < 0.5 {
                                settings_state.video_settings.brightness = 0.5;
                            }
                            // TODO: Apply brightness to the renderer
                        }
                        VideoSettingControl::Contrast => {
                            // Cycle contrast between 0.5 and 1.5 in 0.1 increments
                            settings_state.video_settings.contrast = (settings_state.video_settings.contrast + 0.1) % 1.6;
                            if settings_state.video_settings.contrast < 0.5 {
                                settings_state.video_settings.contrast = 0.5;
                            }
                            // TODO: Apply contrast to the renderer
                        }
                        VideoSettingControl::VSync => {
                            settings_state.video_settings.vsync = !settings_state.video_settings.vsync;
                            // TODO: Apply VSync setting to the renderer
                        }
                        VideoSettingControl::FpsLimit => {
                            // Cycle through common FPS limits
                            let fps_limits = [None, Some(30), Some(60), Some(120), Some(144), Some(240)];
                            
                            if let Some(pos) = fps_limits.iter().position(|&f| f == settings_state.video_settings.fps_limit) {
                                let next_pos = (pos + 1) % fps_limits.len();
                                settings_state.video_settings.fps_limit = fps_limits[next_pos];
                            } else {
                                settings_state.video_settings.fps_limit = Some(60); // Default to 60 FPS
                            }
                            // TODO: Apply FPS limit to the renderer
                        }
                        VideoSettingControl::UiScale => {
                            // Cycle through UI scale factors
                            let scales = [0.75, 1.0, 1.25, 1.5, 2.0];
                            
                            if let Some(pos) = scales.iter().position(|&s| (s - settings_state.video_settings.ui_scale).abs() < f32::EPSILON) {
                                let next_pos = (pos + 1) % scales.len();
                                settings_state.video_settings.ui_scale = scales[next_pos];
                            } else {
                                settings_state.video_settings.ui_scale = 1.0; // Default to 100%
                            }
                            // TODO: Apply UI scale to the UI system
                        }
                    }
                }
            }
        }
    }
}

pub fn cleanup_menu<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        if let Some(entity_commands) = commands.get_entity(entity) {
            entity_commands.despawn_recursive();
        }
    }
}
