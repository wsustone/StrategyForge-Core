use bevy::prelude::*;

// Marker component for the settings menu
#[derive(Component, Debug, Default)]
pub struct SettingsMenuMarker;

impl SettingsMenuMarker {
    pub fn new() -> Self {
        Self
    }
}

// Marker components for buttons
#[derive(Component, Debug, Default)]
pub struct ApplyButton;

#[derive(Component, Debug, Default)]
pub struct ResetButton;

#[derive(Component, Debug, Default)]
pub struct BackButton;

// Tab button component
#[derive(Component, Debug, Clone, Copy)]
pub struct TabButton {
    pub tab: u8,
}

impl TabButton {
    pub fn new(tab: u8) -> Self {
        Self { tab }
    }
}

impl Default for TabButton {
    fn default() -> Self {
        Self { tab: 0 }
    }
}

// Tab content marker
#[derive(Component, Debug)]
pub struct TabContent {
    pub tab: u8,
}

impl TabContent {
    pub fn new(tab: u8) -> Self {
        Self { tab }
    }
}

impl Default for TabContent {
    fn default() -> Self {
        Self { tab: 0 }
    }
}

// Settings tab enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub enum SettingsTab {
    Video = 0,
    Audio = 1,
    Controls = 2,
    Gameplay = 3,
    Graphics = 4,
    Accessibility = 5,
    Other = 6,
}

// Video settings controls
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum VideoSettingControl {
    DisplayMode,
    Resolution,
    GraphicsQuality,
    Brightness,
    Contrast,
    VSync,
    FpsLimit,
    UiScale,
}

impl Default for VideoSettingControl {
    fn default() -> Self {
        Self::DisplayMode
    }
}

// Settings state resource
#[derive(Resource, Default, Debug)]
pub struct SettingsState {
    pub current_tab: u8,
    pub video_settings: VideoSettings,
    // TODO: Add other settings state fields here as needed
}

// Video settings
#[derive(Default, Debug, Clone, Copy)]
pub struct VideoSettings {
    pub display_mode: DisplayMode,
    pub resolution: (u32, u32),
    pub graphics_quality: GraphicsQuality,
    pub brightness: f32,
    pub contrast: f32,
    pub vsync: bool,
    pub fps_limit: Option<u32>,
    pub ui_scale: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub enum DisplayMode {
    Windowed,
    Fullscreen,
    Borderless,
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::Windowed
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Component, Reflect)]
#[reflect(Component)]
pub enum GraphicsQuality {
    Low,
    Medium,
    High,
    Ultra,
    Custom,
}

impl Default for GraphicsQuality {
    fn default() -> Self {
        Self::High
    }
}

// Setting control component
#[derive(Component, Debug)]
pub struct SettingControl {
    pub setting_type: SettingType,
}

impl SettingControl {
    pub fn new(setting_type: SettingType) -> Self {
        Self { setting_type }
    }
    
    pub fn as_video_setting_control(&self) -> Option<&VideoSettingControl> {
        if let SettingType::Video(control) = &self.setting_type {
            Some(control)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SettingType {
    Video(VideoSettingControl),
    Audio,
    Controls,
    Gameplay,
    Graphics,
    Accessibility,
    Other,
}

impl Default for SettingType {
    fn default() -> Self {
        Self::Video(VideoSettingControl::Resolution)
    }
}

// Events
#[derive(Event)]
pub struct SettingsChangedEvent {
    pub setting_type: SettingType,
    // TODO: Add fields for the changed setting values
}

#[derive(Event)]
pub struct ApplySettingsEvent;

#[derive(Event)]
pub struct ResetSettingsEvent;

#[derive(Event)]
pub struct BackToMenuEvent;
