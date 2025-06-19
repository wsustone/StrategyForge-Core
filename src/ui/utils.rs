//! Utility functions and helpers for UI

use bevy::prelude::*;
use crate::ButtonHoverEffect;

/// Creates a text bundle with the given style
pub fn create_text(
    text: impl Into<String>,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size,
            color,
        },
    )
    .with_style(Style {
        margin: UiRect::all(Val::Px(5.0)),
        ..Default::default()
    })
}

/// Creates a panel with the given theme
pub fn create_panel(theme: &crate::ui::theme::PanelTheme) -> impl Bundle {
    let mut style = Style {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::FlexStart,
        align_items: AlignItems::Stretch,
        padding: UiRect::all(Val::Px(theme.padding)),
        ..Default::default()
    };

    // If we have a border, add some margin to account for it
    if theme.border.is_some() {
        style.margin = UiRect::all(Val::Px(theme.border_thickness));
    }

    let mut node_bundle = NodeBundle {
        style,
        background_color: theme.background.into(),
        ..Default::default()
    };

    // Apply border if specified
    if let Some(border_color) = theme.border {
        node_bundle.border_color = border_color.into();
    }

    node_bundle
}

/// Creates a tab button with the given theme
pub fn create_tab_button(
    theme: &crate::ui::theme::TabTheme,
    text: impl Into<String>,
    font: Handle<Font>,
    selected: bool,
) -> (ButtonBundle, ButtonHoverEffect) {
    let _ = (text, font); // Mark as used
    let background_color = if selected {
        theme.selected
    } else {
        theme.normal
    };

    let _text_color = if selected {
        theme.selected_text
    } else {
        theme.text
    };

    let button_bundle = ButtonBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(10.0)),
            margin: UiRect::horizontal(Val::Px(2.0)),
            ..Default::default()
        },
        background_color: background_color.into(),
        ..Default::default()
    };

    let hover_effect = ButtonHoverEffect {
        normal: background_color,
        hovered: if selected { theme.selected } else { theme.hovered },
        pressed: theme.selected,
    };

    (button_bundle, hover_effect)
}

/// Creates a scroll view with a scroll bar
pub fn create_scroll_view(
    width: Val,
    height: Val,
    theme: &crate::ui::theme::PanelTheme,
) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            width,
            height,
            overflow: Overflow::clip(),
            ..Default::default()
        },
        background_color: theme.background.into(),
        ..Default::default()
    }
}

/// Helper trait for UI node builders
trait UiBuilderExt {
    /// Set the position of the UI node
    fn with_position(self, position: UiRect) -> Self;
    
    /// Set the size of the UI node
    fn with_size(self, width: Val, height: Val) -> Self;
    
    /// Set the background color of the UI node
    fn with_background_color(self, color: Color) -> Self;
}

impl UiBuilderExt for NodeBundle {
    fn with_position(mut self, position: UiRect) -> Self {
        self.style.position_type = PositionType::Absolute;
        self.style.left = position.left;
        self.style.right = position.right;
        self.style.top = position.top;
        self.style.bottom = position.bottom;
        self
    }
    
    fn with_size(mut self, width: Val, height: Val) -> Self {
        self.style.width = width;
        self.style.height = height;
        self
    }
    
    fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = color.into();
        self
    }
}
