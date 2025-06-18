//! UI theme and color schemes for StrategyForge

use bevy::prelude::*;
use bevy::ecs::system::Resource;

/// UI theme colors
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct UiTheme {
    /// Primary background color
    pub background: Color,
    /// Secondary background color (for panels, etc.)
    pub panel: Color,
    /// Primary text color
    pub text: Color,
    /// Secondary text color (for labels, etc.)
    pub text_secondary: Color,
    /// Accent color (for highlights, buttons, etc.)
    pub accent: Color,
    /// Success color
    pub success: Color,
    /// Warning color
    pub warning: Color,
    /// Error color
    pub error: Color,
    /// Disabled opacity
    pub disabled_opacity: f32,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            background: Color::srgb(0.1, 0.1, 0.15),
            panel: Color::srgb(0.15, 0.15, 0.2),
            text: Color::srgb(0.9, 0.9, 0.95),
            text_secondary: Color::srgb(0.7, 0.7, 0.8),
            accent: Color::srgb(0.3, 0.5, 0.8),
            success: Color::srgb(0.2, 0.8, 0.2),
            warning: Color::srgb(0.9, 0.7, 0.1),
            error: Color::srgb(0.9, 0.2, 0.2),
            disabled_opacity: 0.5,
        }
    }
}

/// Button theme
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct ButtonTheme {
    /// Normal state color
    pub normal: Color,
    /// Hovered state color
    pub hovered: Color,
    /// Pressed state color
    pub pressed: Color,
    /// Disabled state color
    pub disabled: Color,
    /// Text color
    pub text: Color,
    /// Border color
    pub border: Option<Color>,
    /// Border thickness
    pub border_thickness: f32,
    /// Corner radius
    pub corner_radius: f32,
}

impl Default for ButtonTheme {
    fn default() -> Self {
        let theme = UiTheme::default();
        Self {
            normal: theme.panel,
            hovered: Color::srgb(0.25, 0.25, 0.3),
            pressed: Color::srgb(0.35, 0.35, 0.4),
            disabled: Color::srgba(0.2, 0.2, 0.25, 0.5),
            text: theme.text,
            border: None,
            border_thickness: 1.0,
            corner_radius: 4.0,
        }
    }
}

/// Panel theme
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct PanelTheme {
    /// Background color
    pub background: Color,
    /// Border color
    pub border: Option<Color>,
    /// Border thickness
    pub border_thickness: f32,
    /// Corner radius
    pub corner_radius: f32,
    /// Padding
    pub padding: f32,
}

impl Default for PanelTheme {
    fn default() -> Self {
        let theme = UiTheme::default();
        Self {
            background: theme.panel,
            border: Some(Color::srgba(1.0, 1.0, 1.0, 0.1)),
            border_thickness: 1.0,
            corner_radius: 4.0,
            padding: 10.0,
        }
    }
}

/// Tab theme
#[derive(Debug, Clone, Reflect)]
#[reflect(Debug)]
pub struct TabTheme {
    /// Normal tab color
    pub normal: Color,
    /// Hovered tab color
    pub hovered: Color,
    /// Selected tab color
    pub selected: Color,
    /// Tab text color
    pub text: Color,
    /// Selected tab text color
    pub selected_text: Color,
    /// Tab border color
    pub border: Option<Color>,
    /// Tab border thickness
    pub border_thickness: f32,
    /// Tab corner radius
    pub corner_radius: f32,
}

impl Default for TabTheme {
    fn default() -> Self {
        let theme = UiTheme::default();
        Self {
            normal: Color::srgb(0.15, 0.15, 0.2),
            hovered: Color::srgb(0.2, 0.2, 0.25),
            selected: theme.accent,
            text: theme.text_secondary,
            selected_text: theme.text,
            border: Some(Color::srgba(1.0, 1.0, 1.0, 0.1)),
            border_thickness: 1.0,
            corner_radius: 4.0,
        }
    }
}

/// Complete UI theme
#[derive(Debug, Clone, Reflect, Resource)]
#[reflect(Debug)]
pub struct Theme {
    /// Base UI theme
    pub ui: UiTheme,
    /// Button theme
    pub button: ButtonTheme,
    /// Panel theme
    pub panel: PanelTheme,
    /// Tab theme
    pub tab: TabTheme,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            ui: UiTheme::default(),
            button: ButtonTheme::default(),
            panel: PanelTheme::default(),
            tab: TabTheme::default(),
        }
    }
}

/// Plugin that sets up the default theme
pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<UiTheme>()
            .register_type::<ButtonTheme>()
            .register_type::<PanelTheme>()
            .register_type::<TabTheme>()
            .register_type::<Theme>()
            .insert_resource(Theme::default());
    }
}
