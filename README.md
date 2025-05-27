# StrategyForge Core

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Crates.io](https://img.shields.io/crates/v/strategyforge-core.svg)](https://crates.io/crates/strategyforge-core)
[![Docs](https://docs.rs/strategyforge-core/badge.svg)](https://docs.rs/strategyforge-core)

Core game engine and systems for StrategyForge, a modular real-time strategy game.

## Features

- 🎮 Core game loop and state management
 - Modular plugin system
 - 🛠️ Entity Component System (ECS) powered by Bevy
 - 📦 Resource management
 - 🔄 Event system
 - 🎛️ Input handling
 - 🖥️ Window and display management

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
strategyforge-core = { git = "https://github.com/wsustone/StrategyForge-Core.git" }
```

### Basic Usage

```rust
use bevy::prelude::*;
use strategyforge_core::{CorePlugin, state::GameState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugin)
        .run();
}
```

## Plugin System

StrategyForge Core provides a powerful plugin system for extending game functionality:

```rust
use bevy::prelude::*;
use strategyforge_core::plugin::GamePlugin;

#[derive(Default)]
struct MyGamePlugin;

impl Plugin for MyGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_my_plugin);
    }
}

fn setup_my_plugin() {
    println!("My plugin is loaded!");
}
```

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Documentation

Generate documentation:

```bash
cargo doc --open
```

## Contributing

Contributions are welcome! Please read our [contributing guidelines](CONTRIBUTING.md) before submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Bevy Engine](https://bevyengine.org/)
- All contributors and supporters
