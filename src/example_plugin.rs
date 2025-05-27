use bevy::prelude::*;
use crate::plugin::GamePlugin;

/// Example plugin demonstrating the plugin system
#[derive(Clone)]
pub struct ExamplePlugin;

impl bevy::prelude::Plugin for ExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_example_plugin);
    }
}

impl GamePlugin for ExamplePlugin {
    fn name(&self) -> &'static str {
        "Example Plugin"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }
}

fn setup_example_plugin(mut commands: Commands) {
    info!("Example plugin initialized!");
}
