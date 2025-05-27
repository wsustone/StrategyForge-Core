# Settings Menu

The settings menu allows players to customize their game experience across various categories.

## Categories

### Video Settings
- **Display Mode**: Windowed, Fullscreen, Borderless
- **Resolution**: Screen resolution options
- **Graphics Quality**: Presets (Low, Medium, High, Ultra) and custom settings
- **Brightness/Contrast**: Adjust display settings
- **VSync**: Enable/disable vertical sync
- **FPS Limit**: Cap frame rate
- **UI Scale**: Adjust the size of UI elements

### Audio Settings
- **Input Device**: Select audio input device
- **Output Device**: Select audio output device
- **Master Volume**: Overall game volume
- **Music Volume**: Background music volume
- **SFX Volume**: Sound effects volume
- **Voice Volume**: Dialogue and voice-over volume
- **Ambient Volume**: Environmental sound volume
- **Mute When Inactive**: Auto-mute when window loses focus

### Controls
- **Key Bindings**: Customize keyboard and mouse controls
- **Controller Setup**: Configure gamepad controls
- **Mouse Sensitivity**: Adjust camera and cursor sensitivity
- **Invert Y-Axis**: Toggle Y-axis inversion
- **Vibration**: Enable/disable controller vibration

### Gameplay
- **Difficulty**: Game difficulty level
- **Tutorials**: Enable/disable tutorial messages
- **Auto-save**: Configure auto-save frequency
- **Language**: Game language selection
- **Subtitles**: Toggle and configure subtitles

### Interface
- **HUD Layout**: Customize heads-up display
- **Minimap**: Configure minimap settings
- **Tooltips**: Enable/disable and configure tooltips
- **Damage Numbers**: Toggle damage number display
- **Health Bars**: Configure unit health bar display

## Design Principles
- Clear categorization of settings
- Consistent layout across all settings panels
- Real-time preview of changes when applicable
- Reset to default option for each category
- Clear indication of unsaved changes
- Support for both mouse and keyboard navigation

## Technical Notes
- Settings are saved to a configuration file
- Changes are applied immediately when possible
- Some settings may require a restart to take effect
- Input validation for all user-modifiable values
- Support for multiple controller types
