//! Components for the main menu

use bevy::prelude::*;

/// Marker component for main menu entities
#[derive(Component)]
pub struct MainMenuMarker;

// Button components
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct NewGameButton;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct LoadGameButton;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct MultiplayerButton;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct SettingsButton;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct CreditsButton;

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ExitButton;
