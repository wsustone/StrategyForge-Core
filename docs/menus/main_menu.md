# Main Menu

The main menu serves as the primary navigation hub for the game. It provides access to all major game features and settings.

## Key Features

1. **New Game**
   - Start a new campaign
   - Select difficulty level
   - Choose game mode (Campaign, Skirmish, Tutorial)

2. **Load Game**
   - View and load saved games
   - See metadata (date, playtime, location)
   - Delete saved games

3. **Multiplayer**
   - Host a new multiplayer game
   - Join existing multiplayer games
   - Browse available servers
   - Manage friends list

4. **Settings**
   - Access to all game settings
   - Audio, video, controls, and gameplay options

5. **Credits**
   - View development team
   - Third-party attributions
   - Special thanks

6. **Exit**
   - Exit to desktop
   - Return to launcher (if applicable)

## Design Principles

- Clear visual hierarchy with primary actions prominently displayed
- Consistent navigation patterns
- Responsive design for different screen sizes
- Keyboard and controller navigation support
- Clear visual feedback on interaction

## Technical Notes

- Uses `bevy_ui` for rendering
- Implements state management for menu transitions
- Supports controller and keyboard navigation
- Saves and loads player preferences
