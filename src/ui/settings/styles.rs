use bevy::{
    prelude::*,
    ui::{BackgroundColor, FlexDirection, AlignItems, JustifyContent, Val, UiRect, Style},
};

// UI Colors
pub const WINDOW_BG_COLOR: Color = Color::srgb(0.1, 0.1, 0.15);
pub const PANEL_BG_COLOR: Color = Color::srgb(0.15, 0.15, 0.2);
pub const BUTTON_COLOR: Color = Color::srgb(0.2, 0.2, 0.3);
pub const BUTTON_HOVER_COLOR: Color = Color::srgb(0.25, 0.25, 0.35);
pub const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.35, 0.35, 0.45);
pub const BUTTON_SELECTED_COLOR: Color = Color::srgb(0.3, 0.3, 0.4);
pub const TEXT_COLOR: Color = Color::WHITE;
pub const TEXT_DISABLED_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

// Constants for UI styling
pub const TAB_BUTTON_SELECTED_COLOR: Color = Color::srgb(0.3, 0.3, 0.4);
pub const TAB_BUTTON_HOVER_COLOR: Color = Color::srgb(0.25, 0.25, 0.35);
pub const TAB_BUTTON_NORMAL_COLOR: Color = Color::srgb(0.2, 0.2, 0.3);

/// Creates a text style for titles
pub fn title_text_style(_asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
    TextStyle {
        font: default(), // Use Bevy's default font
        font_size,
        color: Color::WHITE,
    }
}

/// Creates a text style for regular text
pub fn regular_text_style(_asset_server: &Res<AssetServer>, font_size: f32) -> TextStyle {
    TextStyle {
        font: default(), // Use Bevy's default font
        font_size,
        color: Color::WHITE,
    }
}

/// Creates a button bundle with the given text and style
pub fn create_text_button_bundle(
    text: &str,
    text_style: TextStyle,
    is_selected: bool,
) -> (ButtonBundle, TextBundle) {
    let button_bundle = ButtonBundle {
        style: Style {
            padding: UiRect::all(Val::Px(10.0)),
            min_width: Val::Px(100.0),
            min_height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: if is_selected {
            BackgroundColor(BUTTON_SELECTED_COLOR)
        } else {
            BackgroundColor(BUTTON_COLOR)
        },
        ..default()
    };

    let text_bundle = TextBundle::from_section(text, text_style);

    (button_bundle, text_bundle)
}

/// Creates a section container with the given style
pub fn create_section_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2)),
        ..default()
    }
}
