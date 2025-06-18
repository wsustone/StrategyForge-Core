use bevy::prelude::*;

/// Main game state
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    /// Initial loading state
    #[default]
    Loading,
    
    /// Main menu screen
    MainMenu,
    
    /// Settings menu
    Settings,
    
    /// Campaign selection and management
    CampaignMenu,
    
    /// Single player game mode
    Singleplayer,
    
    /// Multiplayer game mode
    Multiplayer,
    
    /// In-game menu (paused)
    Paused,
    
    /// Game over screen
    GameOver,
    
    /// Credits screen
    Credits,
}
