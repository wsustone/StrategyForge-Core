//! Common UI components for StrategyForge

use bevy::prelude::*;

/// Component for buttons with hover effects
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ButtonHoverEffect {
    /// Normal state color
    pub normal: Color,
    /// Hovered state color
    pub hovered: Color,
    /// Pressed state color
    pub pressed: Color,
}

impl Default for ButtonHoverEffect {
    fn default() -> Self {
        Self {
            normal: Color::srgb(0.15, 0.15, 0.2),
            hovered: Color::srgb(0.25, 0.25, 0.3),
            pressed: Color::srgb(0.35, 0.35, 0.4),
        }
    }
}

/// Component for UI panels
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct Panel {
    /// Whether the panel should have a border
    pub has_border: bool,
    /// Border color if has_border is true
    pub border_color: Option<Color>,
    /// Border thickness if has_border is true
    pub border_thickness: f32,
}

/// Component for UI windows
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct WindowPanel {
    /// Window title
    pub title: String,
    /// Whether the window is draggable
    pub draggable: bool,
    /// Whether the window is resizable
    pub resizable: bool,
    /// Whether the window has a close button
    pub close_button: bool,
}

/// Component for tab buttons
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct TabButton {
    /// Tab identifier
    pub id: String,
    /// Whether the tab is currently selected
    pub selected: bool,
}

impl Default for TabButton {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            selected: false,
        }
    }
}

/// Marker component for scrollable lists
#[derive(Component, Debug, Default, Clone, Reflect)]
#[reflect(Component)]
pub struct ScrollableList;

/// Component for tooltips
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Tooltip {
    /// Tooltip text
    pub text: String,
    /// Tooltip position offset
    pub offset: Vec2,
}

impl Default for Tooltip {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            offset: Vec2::new(0.0, 20.0),
        }
    }
}

/// System to handle button hover effects
pub fn button_effect_system(
    mut interaction_query: Query<(
        &Interaction,
        &mut BackgroundColor,
        &ButtonHoverEffect,
    ), Changed<Interaction>>,
) {
    for (interaction, mut color, effect) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = effect.pressed.into();
            }
            Interaction::Hovered => {
                *color = effect.hovered.into();
            }
            Interaction::None => {
                *color = effect.normal.into();
            }
        }
    }
}
