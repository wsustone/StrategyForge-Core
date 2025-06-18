use bevy::{
    prelude::*,
    app::App,
    log::info,

    render::texture::ImagePlugin,
};

mod state;
mod ui;

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
                    title: "StrategyForge".to_string(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()) // Pixel perfect rendering
    )
    .add_plugins((
        // Add core plugin
        CorePlugin,
        // Add UI plugin
        ui::UIPlugin,
    ));
    
    // Set the initial game state
    app.insert_state(GameState::MainMenu);
    
    // Run the app
    app.run();
}
