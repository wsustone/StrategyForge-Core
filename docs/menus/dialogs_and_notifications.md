# Dialogs and Notifications

This document outlines the various dialog boxes and notification systems used throughout the game for player interaction and feedback.

## Dialog Types

### Confirmation Dialogs
- **Purpose**: Confirm critical actions
- **Usage Examples**:
  - Quitting the game
  - Overwriting save files
  - Abandoning missions
- **Components**:
  - Clear question/message
  - Confirm button (e.g., "Yes", "Delete")
  - Cancel button (e.g., "No", "Cancel")

### Information Dialogs
- **Purpose**: Display important information
- **Usage Examples**:
  - Tutorial messages
  - Game updates
  - System notifications
- **Components**:
  - Informative text
  - Acknowledge button ("OK")
  - Optional "Don't show again" checkbox

### Input Dialogs
- **Purpose**: Collect player input
- **Usage Examples**:
  - Save game names
  - Multiplayer player names
  - Chat messages
- **Components**:
  - Input field
  - Submit button
  - Cancel button
  - Input validation

### Progress Dialogs
- **Purpose**: Show operation progress
- **Usage Examples**:
  - Game saving/loading
  - Level loading
  - Content installation
- **Components**:
  - Progress bar
  - Status text
  - Optional cancel button
  - Estimated time remaining

## Notification System

### Toast Notifications
- **Appearance**: Small, temporary popups
- **Duration**: 3-5 seconds
- **Usage Examples**:
  - Achievement unlocked
  - New quest received
  - Resource collection
  - Ability cooldown

### In-Game Alerts
- **Appearance**: Prominent but non-blocking
- **Persistence**: Until dismissed or condition changes
- **Usage Examples**:
  - Under attack
  - Low resources
  - Unit under attack
  - Mission objective updates

### Chat System
- **Types**:
  - Global chat
  - Team chat
  - Private messages
  - System messages
- **Features**:
  - Chat history
  - Player mentions
  - Emoji support
  - Chat commands

## Design Guidelines

### Visual Hierarchy
- **Critical**: Red/High contrast
- **Warning**: Yellow/Orange
- **Info**: Blue/Neutral
- **Success**: Green

### Accessibility
- Adjustable text size
- High contrast mode
- Screen reader support
- Color-blind friendly palettes

### Animation
- Smooth transitions
- Non-distracting effects
- Clear entry/exit animations
- Audio cues for important notifications

## Technical Implementation
- Event-driven architecture
- Queue system for multiple notifications
- Localization support
- Performance optimization for mobile platforms
- Save/load state for persistent dialogs

## Best Practices
1. Keep messages clear and concise
2. Use consistent terminology
3. Provide clear action buttons
4. Allow keyboard navigation
5. Include tooltips for icons
6. Support controller input
7. Test on all target platforms
8. Consider touch targets for mobile
9. Implement proper focus management
10. Include accessibility features
