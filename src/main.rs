use bevy::prelude::*;
use strategyforge_core::StrategyForgeCorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(StrategyForgeCorePlugin)
        .run();
}
