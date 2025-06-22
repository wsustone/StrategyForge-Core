use bevy::{
    prelude::*,
    window::PrimaryWindow,
    input::mouse::{MouseButton, MouseWheel, MouseScrollUnit},
    input::ButtonInput,
    render::camera::Camera,
    math::{Vec2, Vec3},
    ecs::query::{With, Without},
};

use crate::world::WorldConfig;

/// Camera settings that can be configured
#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct CameraSettings {
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub pan_speed: f32,
    pub edge_scroll_margin: f32,
    pub edge_scroll_speed: f32,
    pub smooth_speed: f32,
    pub zoom_smooth_speed: f32,
    pub show_debug: bool,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            zoom_speed: 0.1,
            min_zoom: 0.1,
            max_zoom: 5.0,
            pan_speed: 5.0,
            edge_scroll_margin: 50.0,
            edge_scroll_speed: 10.0,
            smooth_speed: 10.0,
            zoom_smooth_speed: 5.0,
            show_debug: false,
        }
    }
}

/// Component for the main game camera
#[derive(Component, Debug, Clone)]
pub struct GameCamera {
    pub target_zoom: f32,
    pub zoom_speed: f32,
    pub min_zoom: f32,
    pub max_zoom: f32,
    pub drag_start: Option<Vec2>,
    pub pan_speed: f32,
    pub target_position: Vec3,
}

// Update the camera pan system to use smoothing
pub fn camera_pan_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera), With<Camera>>,
    camera_settings: Res<CameraSettings>,
    world_config: Res<WorldConfig>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
) {
    let window = window_query.single();
    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let cursor_pos = window.cursor_position().unwrap_or(window_size * 0.5);
    
    // Process each camera
    for (transform, mut camera) in camera_query.iter_mut() {
        let mut target_position = camera.target_position;
        let mut should_update = false;
        
        // Handle middle mouse button drag
        if mouse_buttons.pressed(MouseButton::Middle) {
            if let Some(cursor_delta) = window.cursor_position() {
                if let Some(drag_start) = camera.drag_start {
                    let delta = (drag_start - cursor_delta) * camera.pan_speed * transform.scale.x;
                    target_position.x += delta.x;
                    target_position.y += delta.y;
                    should_update = true;
                }
                camera.drag_start = Some(cursor_delta);
            }
        } else {
            camera.drag_start = None;
            
            // Edge scrolling
            let edge_margin = camera_settings.edge_scroll_margin;
            let mut scroll_direction = Vec2::ZERO;
            
            if cursor_pos.x < edge_margin {
                scroll_direction.x = -1.0;
            } else if cursor_pos.x > window_size.x - edge_margin {
                scroll_direction.x = 1.0;
            }
            
            if cursor_pos.y < edge_margin {
                scroll_direction.y = 1.0;
            } else if cursor_pos.y > window_size.y - edge_margin {
                scroll_direction.y = -1.0;
            }
            
            if scroll_direction != Vec2::ZERO {
                let scroll_speed = camera_settings.edge_scroll_speed * time.delta_seconds() * transform.scale.x;
                target_position.x += scroll_direction.x * scroll_speed;
                target_position.y += scroll_direction.y * scroll_speed;
                should_update = true;
            }
        }
        
        if should_update {
            // Clamp target position to map bounds
            let viewport_width = (window.width() / transform.scale.x).max(1.0);
            let viewport_height = (window.height() / transform.scale.y).max(1.0);
            
            let min_x = viewport_width / 2.0;
            let max_x = (world_config.width_meters - viewport_width / 2.0).max(min_x + 1.0);
            let min_y = viewport_height / 2.0;
            let max_y = (world_config.height_meters - viewport_height / 2.0).max(min_y + 1.0);
            
            camera.target_position.x = target_position.x.clamp(min_x, max_x);
            camera.target_position.y = target_position.y.clamp(min_y, max_y);
        }
    }
}

pub fn apply_camera_smoothing(
    time: Res<Time>,
    camera_settings: Res<CameraSettings>,
    mut camera_query: Query<(&mut Transform, &GameCamera), With<Camera>>,
) {
    for (mut transform, camera) in camera_query.iter_mut() {
        // Smooth position
        let target_position = camera.target_position;
        let current_position = transform.translation;
        
        // Only interpolate if we're not already very close
        if current_position.distance_squared(target_position) > 0.0001 {
            let new_position = current_position.lerp(
                target_position,
                camera_settings.smooth_speed * time.delta_seconds()
            );
            transform.translation = new_position;
        } else {
            transform.translation = target_position;
        }
        
        // Smooth zoom
        let target_zoom = camera.target_zoom;
        let current_scale = transform.scale.x;
        
        if (current_scale - target_zoom).abs() > 0.001 {
            let new_scale = current_scale.lerp(
                target_zoom,
                camera_settings.zoom_smooth_speed * time.delta_seconds()
            );
            transform.scale = Vec3::splat(new_scale);
        } else {
            transform.scale = Vec3::splat(target_zoom);
        }
    }
}

/// System to handle camera zoom with mouse wheel
pub fn camera_zoom_system(
    mut wheel_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut GameCamera>,
    _camera_settings: Res<CameraSettings>,
) {
    for mut camera in camera_query.iter_mut() {
        let mut zoom_delta = 0.0;
        
        for event in wheel_events.read() {
            match event.unit {
                MouseScrollUnit::Line => zoom_delta += event.y * 0.1,
                MouseScrollUnit::Pixel => zoom_delta += event.y * 0.01,
            }
        }
        
        if zoom_delta != 0.0 {
            // Calculate zoom factor (inverse because scrolling up should zoom in)
            let zoom_factor = 1.0 - zoom_delta * camera.zoom_speed;
            let new_zoom = (camera.target_zoom * zoom_factor)
                .clamp(camera.min_zoom, camera.max_zoom);
            
            camera.target_zoom = new_zoom;
        }
    }
}

/// Plugin for camera controls
// Component for the debug overlay
#[derive(Component)]
struct CameraDebugOverlay;

// System to set up the debug overlay
fn setup_camera_debug_overlay(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_settings: Res<CameraSettings>,
) {
    if !camera_settings.show_debug {
        return;
    }
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Camera Debug",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            transform: Transform::from_xyz(10.0, 30.0, 100.0),
            ..default()
        },
        CameraDebugOverlay,
    ));
}

// System to update the debug overlay
fn update_camera_debug_overlay(
    camera_query: Query<(&Transform, &GameCamera), With<Camera>>,
    mut text_query: Query<&mut Text, With<CameraDebugOverlay>>,
    camera_settings: Res<CameraSettings>,
) {
    if !camera_settings.show_debug {
        return;
    }
    
    if let Ok((transform, camera)) = camera_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!(
                "Position: ({:.1}, {:.1})\nZoom: {:.2}\nTarget Zoom: {:.2}",
                transform.translation.x,
                transform.translation.y,
                transform.scale.x,
                camera.target_zoom
            );
        }
    }
}

pub struct CameraControlsPlugin;

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<CameraSettings>()
            .init_resource::<CameraSettings>()
            .add_systems(Startup, (
                setup_camera_controls.after(crate::camera::setup_game_camera),
                setup_camera_debug_overlay,
            ))
            .add_systems(
                Update,
                (
                    camera_pan_system,
                    camera_zoom_system,
                    apply_camera_smoothing.after(camera_zoom_system).after(camera_pan_system),
                    update_camera_debug_overlay,
                )
            );
    }
}

/// System to set up the camera controls
fn setup_camera_controls(
    mut commands: Commands,
    camera_settings: Res<CameraSettings>,
    world_config: Res<WorldConfig>,
    mut camera_query: Query<Entity, (With<Camera>, Without<GameCamera>)>,
) {
    // Get the existing game camera
    if let Ok(camera_entity) = camera_query.get_single_mut() {
        // Add GameCamera component to the existing camera
        commands.entity(camera_entity).insert(GameCamera {
            target_zoom: 1.0,
            zoom_speed: camera_settings.zoom_speed,
            min_zoom: camera_settings.min_zoom,
            max_zoom: camera_settings.max_zoom,
            drag_start: None,
            pan_speed: camera_settings.pan_speed,
            target_position: Vec3::new(
                world_config.width_meters / 2.0,
                world_config.height_meters / 2.0,
                0.0
            ),
        });
    }
}
