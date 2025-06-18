//! Common UI styles and style builders for StrategyForge

use bevy::{
    prelude::*,
    ui::{
        AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val,
        BackgroundColor, Overflow,
    },
    text::TextStyle,
};

// Helper function to set background color
fn background_color(color: Color) -> BackgroundColor {
    BackgroundColor(color)
}

// Helper function to create a style with default values
const fn default_style() -> Style {
    Style::DEFAULT
}

/// Common button style
pub fn button_style() -> Style {
    let mut style = default_style();
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.padding = UiRect::all(Val::Px(10.0));
    style.margin = UiRect::all(Val::Px(5.0));
    style
}

/// Common text style for buttons
pub fn button_text_style() -> TextStyle {
    TextStyle {
        font_size: 24.0,
        ..Default::default()
    }
}

/// Common panel style
pub fn panel_style() -> Style {
    let mut style = default_style();
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::FlexStart;
    style.align_items = AlignItems::Center;
    style.padding = UiRect::all(Val::Px(20.0));
    style.margin = UiRect::all(Val::Px(10.0));
    style
}

/// Common window style
pub fn window_style() -> Style {
    let mut style = default_style();
    style.position_type = PositionType::Absolute;
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::FlexStart;
    style.align_items = AlignItems::Stretch;
    style.padding = UiRect::all(Val::Px(10.0));
    style
}

/// Common title bar style
pub fn title_bar_style() -> Style {
    let mut style = default_style();
    style.flex_direction = FlexDirection::Row;
    style.justify_content = JustifyContent::SpaceBetween;
    style.align_items = AlignItems::Center;
    style.padding = UiRect::all(Val::Px(5.0));
    style
}

/// Common tab button style
pub fn tab_button_style() -> Style {
    let mut style = default_style();
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.padding = UiRect::all(Val::Px(10.0));
    style.margin = UiRect::horizontal(Val::Px(2.0));
    style
}

/// Common tab content style
pub fn tab_content_style() -> Style {
    let mut style = default_style();
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::FlexStart;
    style.align_items = AlignItems::FlexStart;
    style.padding = UiRect::all(Val::Px(10.0));
    style
}

/// Common label style
pub fn label_style() -> Style {
    let mut style = default_style();
    style.margin = UiRect::all(Val::Px(5.0));
    style
}

/// Common text input style
pub fn text_input_style() -> Style {
    let mut style = default_style();
    style.margin = UiRect::all(Val::Px(5.0));
    style.padding = UiRect::all(Val::Px(5.0));
    style
}

/// Common scroll view style
pub fn scroll_view_style() -> Style {
    let mut style = default_style();
    style.flex_direction = FlexDirection::Column;
    style.justify_content = JustifyContent::FlexStart;
    style.align_items = AlignItems::Stretch;
    style.overflow = Overflow::clip();
    style
}

/// Common scroll bar style
pub fn scroll_bar_style() -> Style {
    let mut style = default_style();
    style.position_type = PositionType::Absolute;
    style.right = Val::Px(5.0);
    style.top = Val::Px(5.0);
    style.bottom = Val::Px(5.0);
    style.width = Val::Px(10.0);
    style
}

/// Common tooltip style
pub fn tooltip_style() -> Style {
    let mut style = default_style();
    style.position_type = PositionType::Absolute;
    style.padding = UiRect::all(Val::Px(5.0));
    style
}

/// Common style for disabled UI elements
pub fn disabled_style() -> Style {
    let style = default_style();
    // Note: Opacity is now handled by the Alpha component in Bevy 0.14+
    style
}

/// Common style for selected items
pub fn selected_style() -> Style {
    let mut style = default_style();
    style.border = UiRect::all(Val::Px(2.0));
    style
}

/// Common style for hovered items
pub fn hovered_style() -> Style {
    let style = default_style();
    // Background color is now handled by the BackgroundColor component
    style
}
