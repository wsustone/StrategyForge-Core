use bevy::prelude::*;
use bevy::ecs::system::ParamSet;
use sf_plugin_template::MenuItemPlugin;
use sf_plugin_template::{MenuItem, MenuContent};

/// Main menu plugin that loads at game start
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Register the system to set up the main menu
        app.add_systems(Startup, setup_main_menu);
        
        // Add menu registration system, but with direct world access
        app.add_systems(PostStartup, register_menu_plugins);
        
        // Add systems to handle menu interactions
        app.add_systems(Update, handle_menu_item_interaction);
    }
}

// Component to mark the menu container
#[derive(Component)]
pub struct MenuContainer;

// Component to mark the content area
#[derive(Component)]
pub struct ContentArea;

/// Registry of menu plugins
/// This is a manual registry since we can't use Resources for this
/// due to thread-safety concerns
static mut MENU_PLUGINS: Option<Vec<Box<dyn MenuItemPlugin>>> = None;

/// Register a menu plugin
pub fn register_menu_plugin(plugin: Box<dyn MenuItemPlugin>) {
    // Safety: This function should only be called during app setup
    // before any systems run, ensuring no thread-safety issues
    unsafe {
        if MENU_PLUGINS.is_none() {
            MENU_PLUGINS = Some(Vec::new());
        }
        
        if let Some(plugins) = &mut MENU_PLUGINS {
            plugins.push(plugin);
        }
    }
}

/// Set up the main menu structure
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
                    ..default()
                },
                background_color: Color::rgb(0.2, 0.2, 0.3).into(),
                ..default()
            },
            MenuContainer,
            Name::new("MenuContainer"),
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
            
            // Exit button at the bottom of the menu
            parent.spawn(
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Auto,
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::top(Val::Auto), // Push to bottom
                        ..default()
                    },
                    ..default()
                }
            )
            .with_children(|parent| {
                // Exit game button
                parent.spawn(
                    ButtonBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        },
                        background_color: Color::rgb(0.6, 0.2, 0.2).into(),
                        ..default()
                    },
                )
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Exit Game",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
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
                    background_color: Color::rgb(0.2, 0.2, 0.2).into(),
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
                    background_color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..default()
                },
                MenuContent,
                Name::new("ContentDisplay"),
            ))
            .with_children(|parent| {
                // Default welcome content
                parent.spawn(
                    TextBundle::from_section(
                        "Welcome to StrategyForge",
                        TextStyle {
                            font_size: 32.0,
                            color: Color::rgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ),
                );
            });
        });
    });
}

/// System to register menu plugins after main menu is set up
fn register_menu_plugins(world: &mut World) {
    // Find the menu container entity
    let menu_container = {
        let mut query = world.query_filtered::<Entity, With<MenuContainer>>();
        match query.iter(world).next() {
            Some(entity) => entity,
            None => return, // No menu container found
        }
    };
    
    // Safety: We're accessing this in a controlled system
    // and don't modify the vector during this access
    unsafe {
        if let Some(plugins) = &MENU_PLUGINS {
            // Call add_menu_item for each plugin
            for plugin in plugins {
                // We need to clone the plugin reference because we can't pass
                // a reference into the mutable world borrow
                let plugin_clone = plugin.clone();
                plugin_clone.add_menu_item(world, menu_container);
            }
        }
    }
}

/// System to handle interaction with menu items
fn handle_menu_item_interaction(
    mut commands: Commands,
    mut query_set: ParamSet<(
        Query<(&Interaction, &mut BackgroundColor, &MenuItem), (Changed<Interaction>, With<Button>)>,
        Query<(Entity, &mut MenuItem, &mut BackgroundColor)>
    )>,
    content_query: Query<Entity, With<MenuContent>>,
) {
    // First, check for interactions in the first query
    let mut clicked_plugin_name = None;
    
    // Use the first query to check for interactions
    for (interaction, mut color, menu_item) in query_set.p0().iter_mut() {
        if *interaction == Interaction::Pressed {
            // Store the plugin name for processing in the second query
            clicked_plugin_name = Some(menu_item.plugin_name.clone());
        } else if *interaction == Interaction::Hovered {
            // Highlight on hover (if not already selected)
            if !menu_item.selected {
                *color = Color::srgb(0.35, 0.35, 0.35).into();
            }
        } else {
            // Reset color when not interacting (if not selected)
            if !menu_item.selected {
                *color = Color::srgb(0.25, 0.25, 0.25).into();
            }
        }
    }
    
    // If we have a clicked plugin, process it in the second query
    if let Some(plugin_name) = clicked_plugin_name {
        // Update all menu items in the second query
        for (entity, mut item, mut bg_color) in query_set.p1().iter_mut() {
            if item.plugin_name == plugin_name {
                // This is the clicked item, mark as selected
                item.selected = true;
                *bg_color = Color::srgb(0.4, 0.4, 0.5).into();
                
                // Find the content display area
                if let Ok(content_entity) = content_query.get_single() {
                    // Find and call the plugin with this name
                    unsafe {
                        if let Some(plugins) = &MENU_PLUGINS {
                            for plugin in plugins {
                                if plugin.menu_name() == plugin_name {
                                    // Use a command to handle the selection in a deferred way
                                    let plugin_clone = plugin.clone_box();
                                    let content_entity_id = content_entity;
                                    
                                    // Create a one-shot system to apply the plugin's on_selected method
                                    commands.add(move |world: &mut World| {
                                        plugin_clone.on_selected(world, content_entity_id);
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            } else {
                // Other items should be marked as not selected
                item.selected = false;
                *bg_color = Color::srgb(0.25, 0.25, 0.25).into();
            }
        }
    }
}
