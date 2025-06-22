use bevy::{
    prelude::*,
    log::info,
    render::texture::ImagePlugin,
    window::WindowMode,
    log::LogPlugin,
    app::AppExit,
};

mod camera;
mod camera_controls;
mod state;
mod ui;
mod resources;
mod world;

// Re-export commonly used modules
pub use world::*;
pub use camera::*;
pub use camera_controls::*;

// Re-export commonly used items
pub use state::*;
pub use ui::*;

/// Plugin that handles base application functionality
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        // Initialize the game state system
        app.init_state::<state::GameState>();
        
        // Add the core startup system
        app.add_systems(Startup, core_startup_system);
    }
    
    fn ready(&self, _app: &App) -> bool {
        true
    }
    
    fn finish(&self, _app: &mut App) {
        // Cleanup if needed
    }
}

// Simple system to ensure core Bevy schedules are properly initialized
fn core_startup_system() {
    info!("Core systems initialized, ready for plugin loading");
}

fn main() {
    // Create the app
    let mut app = App::new();

    // Set up the default plugins
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "StrategyForge".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: true,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()) // For pixel art
            .set({
                let mut log_plugin = LogPlugin::default();
                log_plugin.level = bevy::log::Level::INFO;
                log_plugin.filter = "wgpu=warn,bevy_render=info,bevy_ecs=info".into();
                log_plugin
            })
    )
    .add_plugins((
        CorePlugin,
        camera::CameraPlugin,
        camera_controls::CameraControlsPlugin,
        world::WorldPlugin,
        ui::UIPlugin,  // UI plugin includes MainMenuPlugin and other UI components
    ));

    // Set the initial game state
    app.insert_state(state::GameState::MainMenu);

    // Debug tools can be added here when needed
    #[cfg(debug_assertions)]
    {
        // Add debug tools here when we resolve the dependency issues
    }
    
    // Always add exit on esc
    app.add_systems(Update, exit_on_esc);

    // Run the app
    app.run();
}



/// System to exit the application when ESC is pressed
fn exit_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
