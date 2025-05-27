use bevy::prelude::*;

/// Main game state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    Gameplay,
    GameOver,
}
