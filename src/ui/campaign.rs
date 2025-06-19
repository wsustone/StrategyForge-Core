use bevy::{
    prelude::*,
    ecs::schedule::IntoSystemConfigs,
    ui::{
        Interaction, BackgroundColor,
        JustifyContent, AlignItems,
    },
};
use bevy::prelude::in_state;
use crate::state::GameState;
use super::button_effect_system;
use super::components::*;
use super::components::ScrollableList;

// Re-export commonly used types
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::hierarchy::ChildBuilder;

/// Plugin for the campaign menu
pub struct CampaignMenuPlugin;

impl Plugin for CampaignMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::CampaignMenu), setup_campaign_menu)
            .add_systems(
                Update,
                (
                    button_effect_system,
                    handle_campaign_button_interactions,
                )
                .run_if(in_state(GameState::CampaignMenu)),
            )
            .add_systems(OnExit(GameState::CampaignMenu), cleanup_menu::<CampaignMenuMarker>);
    }
}

// Marker component for campaign menu entities
#[derive(Component)]
struct CampaignMenuMarker;

// Button components
#[derive(Component, Default)]
struct BackButton;

#[derive(Component)]
struct StartCampaignButton;

#[derive(Component)]
struct MissionButton(usize);

// Campaign data
#[derive(Resource, Default)]
struct CampaignState {
    selected_mission: Option<usize>,
    completed_missions: Vec<usize>,
}

fn setup_campaign_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera for the campaign menu
    commands.spawn((Camera2dBundle::default(), CampaignMenuMarker));
    
    // Root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..default()
            },
            CampaignMenuMarker,
        ))
        .with_children(|parent| {
            // Left side - Mission list
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(30.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::right(Val::Px(20.0)),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::srgba(0.1, 0.1, 0.15, 0.8)),
                        ..default()
                    },
                    CampaignMenuMarker,
                ))
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle::from_section(
                        "CAMPAIGN MISSIONS",
                        TextStyle {
                            font: default(), // Use Bevy's default font
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                    ));

                    // Mission list container with scroll
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_grow: 1.0,
                                    flex_direction: FlexDirection::Column,
                                    overflow: Overflow::clip(),
                                    ..default()
                                },
                                ..default()
                            },
                            ScrollableList,
                        ))
                        .with_children(|parent| {
                            // Spawn example missions
                            // In a real game, this would come from a resource
                            for i in 0..10 {
                                let mission_name = format!("Mission {}", i + 1);
                                // For now, mark all missions after the first as locked
                                let is_locked = i > 0;
                                spawn_mission_button(parent, &asset_server, &mission_name, i, is_locked);
                            }
                        });
                });

            // Right side - Mission details
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_grow: 1.0,
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: Color::srgba(0.1, 0.1, 0.15, 0.8).into(),
                        ..default()
                    },
                    CampaignMenuMarker,
                ))
                .with_children(|parent| {
                    // Mission title
                    parent.spawn(TextBundle::from_section(
                        "Select a Mission",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                    ));

                    // Mission description
                    parent.spawn(TextBundle::from_section(
                        "Select a mission from the list to view details.",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                            font_size: 18.0,
                            color: Color::srgb(0.5, 0.5, 0.5), // GRAY
                        },
                    ));

                    // Mission details container
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_grow: 1.0,
                                margin: UiRect::all(Val::Px(20.0)),
                                padding: UiRect::all(Val::Px(20.0)),
                                border: UiRect::all(Val::Px(2.0)),
                                ..default()
                            },
                            border_color: Color::srgba(1.0, 1.0, 1.0, 0.2).into(),
                            ..default()
                        },
                        CampaignMenuMarker,
                    ));

                    // Bottom buttons
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::FlexEnd,
                                    padding: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            CampaignMenuMarker,
                        ))
                        .with_children(|parent| {
                            // Back button
                            spawn_text_button::<BackButton>(
                                parent,
                                &asset_server,
                                "BACK",
                            );

                            // Start Mission button (initially disabled)
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            padding: UiRect::horizontal(Val::Px(20.0)),
                                            margin: UiRect::left(Val::Px(10.0)),
                                            height: Val::Px(40.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::srgb(0.1, 0.5, 0.1).into(),
                                        ..default()
                                    },
                                    ButtonHoverEffect {
                                        normal: Color::srgb(0.1, 0.5, 0.1),
                                        hovered: Color::srgb(0.2, 0.6, 0.2),
                                        pressed: Color::srgb(0.3, 0.7, 0.3),
                                    },
                                    StartCampaignButton,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "START MISSION",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                                            font_size: 18.0,
                                            color: Color::WHITE,
                                        },
                                    ));
                                });
                        });
                });
        });
}

fn spawn_mission_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    name: &str,
    index: usize,
    is_locked: bool,
) -> Entity {
    let button_entity = parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                padding: UiRect::horizontal(Val::Px(10.0)),
                margin: UiRect::bottom(Val::Px(5.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: if is_locked {
                BackgroundColor(Color::srgb(0.1, 0.1, 0.1))
            } else {
                BackgroundColor(Color::srgb(0.15, 0.15, 0.2))
            },
            ..default()
        },
        if !is_locked {
            ButtonHoverEffect {
                normal: Color::srgb(0.15, 0.15, 0.2),
                hovered: Color::srgb(0.25, 0.25, 0.3),
                pressed: Color::srgb(0.35, 0.35, 0.4),
            }
        } else {
            ButtonHoverEffect {
                normal: Color::srgb(0.1, 0.1, 0.1),
                hovered: Color::srgb(0.1, 0.1, 0.1),
                pressed: Color::srgb(0.1, 0.1, 0.1),
            }
        },
        MissionButton(index),
        Interaction::default(),
    )).with_children(|parent| {
        // Mission name
        parent.spawn(TextBundle::from_section(
            name,   
            TextStyle {
                font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                font_size: 18.0,
                color: if is_locked {
                    Color::srgb(0.25, 0.25, 0.25) // DARK_GRAY
                } else {
                    Color::WHITE
                },
            },
        ));

        // Lock icon or completion status
        if is_locked {
            parent.spawn(TextBundle::from_section(
                "🔒",
                TextStyle {
                    font_size: 20.0,
                    ..default()
                },
            ));
        } else {
            // Could add a checkmark for completed missions here
            parent.spawn(TextBundle::default());
        }
    }).id();
    
    button_entity
}

fn spawn_text_button<T: Component + Default>(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    text: &str,
) -> Entity {
    parent
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(40.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
                ..default()
            },
            ButtonHoverEffect {
                normal: Color::srgb(0.15, 0.15, 0.2),
                hovered: Color::srgb(0.25, 0.25, 0.3),
                pressed: Color::srgb(0.35, 0.35, 0.4),
            },
            T::default(),
            Interaction::default(),
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-SemiBold.ttf"),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
        })
        .id()
}

fn handle_campaign_button_interactions(
    mut interaction_query: Query<(&Interaction, Entity, &mut BackgroundColor, Option<&ButtonHoverEffect>), (Changed<Interaction>, With<Button>)>,
    back_button_query: Query<&BackButton>,
    start_button_query: Query<&StartCampaignButton>,
    mission_button_query: Query<&MissionButton>,
    mut campaign_state: ResMut<CampaignState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, entity, mut bg_color, effect) in interaction_query.iter_mut() {
        // Update button appearance based on interaction state
        if let Some(effect) = effect {
            *bg_color = match interaction {
                Interaction::Pressed => BackgroundColor(effect.pressed),
                Interaction::Hovered => BackgroundColor(effect.hovered),
                Interaction::None => BackgroundColor(effect.normal),
            };
        }

        // Only process button press events
        if *interaction != Interaction::Pressed {
            continue;
        }

        // Handle back button
        if back_button_query.get(entity).is_ok() {
            next_state.set(GameState::MainMenu);
            continue;
        }

        // Handle start mission button
        if start_button_query.get(entity).is_ok() {
            if let Some(_mission_id) = campaign_state.selected_mission {
                // TODO: Start the selected mission
                // Using Singleplayer as the game state since Gameplay doesn't exist
                next_state.set(GameState::Singleplayer);
            }
            continue;
        }

        // Handle mission selection
        if let Ok(mission_button) = mission_button_query.get(entity) {
            campaign_state.selected_mission = Some(mission_button.0);
            // TODO: Update mission details
        }
    }
}

fn cleanup_menu<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
