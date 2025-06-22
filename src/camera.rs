use bevy::prelude::*;
use crate::camera_controls::GameCamera;
use crate::state::GameState;
use crate::world::WorldConfig;

/// Sets up the main game camera
pub fn setup_game_camera(mut commands: Commands, world_config: Res<WorldConfig>) {
    let initial_position = Vec3::new(
        world_config.width_meters / 2.0,
        world_config.height_meters / 2.0,
        0.0
    );
    
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(initial_position)
                .with_scale(Vec3::splat(1.0)),
            projection: OrthographicProjection {
                scale: 1.0,
                far: 1000.0,
                near: -1000.0,
                ..default()
            },
            ..default()
        },
        GameCamera {
            target_zoom: 1.0,
            zoom_speed: 2.0,
            min_zoom: 0.1,
            max_zoom: 5.0,
            drag_start: None,
            pan_speed: 1.0,
            target_position: initial_position,  // Initialize target position
        },
        Name::new("GameCamera"),
    ));
}

/// Sets up a UI camera that's always active
pub fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1, // Render UI on top of the game camera
                ..default()
            },
            ..default()
        },
        UiCameraConfig { show_ui: true },
    ));
}

/// Marker component for the UI camera
#[derive(Component)]
pub struct UiCameraConfig {
    pub show_ui: bool,
}

/// Plugin for camera-related functionality
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_ui_camera)
            .add_systems(OnEnter(GameState::MainMenu), cleanup_game_camera)
            .add_systems(OnEnter(GameState::InGame { is_paused: false }), (cleanup_game_camera, setup_game_camera).chain())
            .add_systems(OnEnter(GameState::Settings), cleanup_game_camera);
    }
}

/// Clean up any existing game cameras
fn cleanup_game_camera(
    mut commands: Commands,
    camera_query: Query<Entity, (With<Camera>, Without<UiCameraConfig>)>,
    game_camera_query: Query<Entity, With<GameCamera>>,
) {
    // Clean up any camera with Camera component (except UI camera)
    for entity in &camera_query {
        commands.entity(entity).despawn_recursive();
    }
    
    // Also clean up any remaining GameCamera components
    for entity in &game_camera_query {
        commands.entity(entity).despawn_recursive();
    }
}
