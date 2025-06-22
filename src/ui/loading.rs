use bevy::prelude::*;
use bevy::app::AppExit;

/// Plugin for loading screen UI
pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(crate::state::GameState::Loading), setup_loading_screen)
            .add_systems(Update, update_loading_screen.run_if(in_state(crate::state::GameState::Loading)))
            .add_systems(OnExit(crate::state::GameState::Loading), cleanup_loading_screen);
    }
}

#[derive(Resource)]
struct LoadingProgress {
    progress: f32,
    message: String,
}

impl Default for LoadingProgress {
    fn default() -> Self {
        Self {
            progress: 0.0,
            message: "Loading...".to_string(),
        }
    }
}

#[derive(Component)]
struct LoadingScreen;

fn setup_loading_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn loading screen UI
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
            background_color: Color::rgba(0.1, 0.1, 0.1, 0.9).into(),
            ..default()
        },
        LoadingScreen,
    )).with_children(|parent| {
        // Loading text
        parent.spawn(TextBundle::from_section(
            "Generating World...",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::WHITE,
            },
        ));

        // Progress bar background
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(400.0),
                    height: Val::Px(30.0),
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
        )).with_children(|parent| {
            // Progress bar fill
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(0.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::GREEN.into(),
                    ..default()
                },
                ProgressBarFill,
            ));
        });

        // Progress text
        parent.spawn((
            TextBundle::from_section(
                "0%",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
            ),
            ProgressText,
        ));

        // Status message
        parent.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "Status: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
                TextSection::new(
                    "Initializing...",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Regular.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ]),
            StatusText,
        ));
    });
}

#[derive(Component)]
struct ProgressBarFill;

#[derive(Component)]
struct ProgressText;

#[derive(Component)]
struct StatusText;

fn update_loading_screen(
    mut progress: ResMut<LoadingProgress>,
    mut progress_bar: Query<&mut Style, With<ProgressBarFill>>,
    mut progress_text: Query<&mut Text, With<ProgressText>>,
    mut status_text: Query<&mut Text, (With<StatusText>, Without<ProgressText>)>,
) {
    // Update progress bar
    if let Ok(mut style) = progress_bar.get_single_mut() {
        style.width = Val::Percent(progress.progress * 100.0);
    }

    // Update progress text
    if let Ok(mut text) = progress_text.get_single_mut() {
        text.sections[0].value = format!("{:.1}%", progress.progress * 100.0);
    }

    // Update status text
    if let Ok(mut text) = status_text.get_single_mut() {
        text.sections[1].value = progress.message.clone();
    }
}

fn cleanup_loading_screen(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// Public API for updating loading progress
pub fn update_loading_progress(
    progress: f32,
    message: &str,
    progress_res: Option<ResMut<LoadingProgress>>,
) {
    if let Some(mut progress_res) = progress_res {
        progress_res.progress = progress.clamp(0.0, 1.0);
        progress_res.message = message.to_string();
    }
}
