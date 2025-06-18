use bevy::prelude::*;
use crate::state::GameState;

/// Main menu plugin that handles the main menu UI and interactions.
/// 
/// This plugin sets up the main menu UI and handles interactions with menu buttons.
/// It manages the transition between different game states based on user input.
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Register the system to set up the main menu
        app.add_systems(Startup, setup_main_menu);
        
        // Add systems to handle menu interactions for static buttons
        app.add_systems(Update, (handle_settings_button_interaction, handle_campaign_button_interaction).run_if(in_state(GameState::MainMenu)));
        // Note: handle_exit_button_interaction could be added here if it's not already part of another system or if it needs specific GameState condition
    }
}

/// Component that marks the main content area of the menu.
/// 
/// This is used to identify the container where dynamic content should be displayed.
#[derive(Component)]
pub struct ContentArea;

/// Component that marks the exit button in the main menu.
/// 
/// Clicking this button will exit the game.
#[derive(Component)]
pub struct ExitButton;

/// Component that marks the settings button in the main menu.
/// 
/// Clicking this button will transition to the settings menu.
#[derive(Component)]
pub struct SettingsButton;

/// Component that marks the campaign button in the main menu.
/// 
/// Clicking this button will transition to the campaign menu.
#[derive(Component)]
pub struct CampaignButton;

/// Sets up the main menu UI structure.
/// 
/// This function creates the visual elements of the main menu, including:
/// - Menu buttons (Play, Campaign, Settings, Exit)
/// - Content area for dynamic content
/// - Background and styling
#[allow(dead_code)]
fn setup_main_menu(mut commands: Commands) {
    // Create a split layout for menu and content
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
        Name::new("MainMenuRoot"),
    ))
    .with_children(|parent| {
        // Left side menu panel
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(20.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Px(10.0)),
                    justify_content: JustifyContent::SpaceBetween, // This pushes the top and bottom sections apart
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.2, 0.3).into(),
                ..default()
            },
            Name::new("MenuPanel"),
        ))
        .with_children(|parent| {
            // Top section: Contains title and menu items
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        ..default()
                    },
                    ..default()
                },
                Name::new("TopSection"),
            ))
            .with_children(|parent| {
                // Menu title
                parent.spawn(
                    TextBundle::from_section(
                        "Main Menu",
                        TextStyle {
                            font_size: 28.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::vertical(Val::Px(20.0)),
                        align_self: AlignSelf::Center,
                        ..default()
                    }),
                );
                
                // Static Settings Button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.3, 0.7).into(), // A distinct color for settings
                        ..default()
                    },
                    SettingsButton, // Marker component
                    Name::new("SettingsButton"),
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(
                        TextBundle::from_section(
                            "Settings",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                    );
                });

                // Campaign Button (Static)
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.3, 0.7, 0.3).into(), // A distinct color for campaign
                        ..default()
                    },
                    CampaignButton, // Marker component
                    Name::new("CampaignButton"),
                ))
                .with_children(|button_parent| {
                    button_parent.spawn(
                        TextBundle::from_section(
                            "Campaign",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                    );
                });

                // Menu items container (plugins will add items here)
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Stretch,
                            margin: UiRect::top(Val::Px(10.0)), // Add some space above dynamic items
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("MenuItemsContainer"), // MenuContainer marker removed
                ));
            });
            
            // Bottom section: Contains exit button
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Stretch,
                        margin: UiRect::top(Val::Px(10.0)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("BottomSection"),
            ))
            .with_children(|parent| {
                // Exit button
                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        background_color: Color::srgb(0.6, 0.2, 0.2).into(),
                        ..default()
                    },
                    ExitButton,
                    Name::new("ExitButton"),
                ))
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Exit Game",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                    );
                });
            });
        });

        // Main content area
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(80.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ContentArea,
            Name::new("ContentArea"),
        ))
        .with_children(|parent| {
            // Title header
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(80.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                    ..default()
                },
            )
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "StrategyForge",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ),
                );
            });

            // Content display area
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                    ..default()
                },
                Name::new("ContentDisplay"), // MenuContent marker removed
            ))
            .with_children(|parent| {
                // Default welcome content



// Default welcome content
                parent.spawn(
                    TextBundle::from_section(
                        "Welcome to StrategyForge",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ),
                );
            }); // Closes .with_children for ContentDisplay
        }); // Closes .with_children for ContentArea
    }); // Closes .with_children for MainMenuRoot
} // Closes setup_main_menu function

/// System to handle interaction with the static Campaign button
fn handle_campaign_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CampaignButton>)>, 
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                // Transition to CampaignMenu state
                next_state.set(GameState::CampaignMenu);
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.4, 0.8, 0.4).into(); // Slightly lighter when hovered
            }
            Interaction::None => {
                *background_color = Color::srgb(0.3, 0.7, 0.3).into(); // Back to normal
            }
        }
    }
}


/// System to handle interaction with the static Settings button
fn handle_settings_button_interaction(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<SettingsButton>)>, 
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                info!("Settings button pressed. Transitioning to GameState::Settings.");
                next_state.set(GameState::Settings);
            }
            Interaction::Hovered => {
                *background_color = Color::srgb(0.4, 0.4, 0.8).into(); // Darken on hover
            }
            Interaction::None => {
                *background_color = Color::srgb(0.3, 0.3, 0.7).into(); // Default color
            }
        }
    }
}

