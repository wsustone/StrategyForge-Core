//! Faction definitions for StrategyForge campaigns

use bevy::prelude::*;
use bevy_reflect::Reflect;

/// Available factions in the game
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect, Default)]
#[reflect(Component)]
pub enum Faction {
    /// Masters of industrial machinery and heavy weaponry
    #[default]
    Mechanists,
    
    /// Pioneers of advanced technology and artificial intelligence
    Synthetics,
    
    /// Masters of mobility and adaptation in harsh environments
    Nomads,
    
    /// Manipulators of energy fields and crystalline technology
    ArcaneEngineers,
    
    /// Profit-driven specialists in modular technology
    CorporateMercenaries,
    
    /// Manipulators of gravity and dark energy
    VoidHarbingers,
    
    /// Hive-mind entities with distributed intelligence
    SwarmCollective,
}

impl Faction {
    /// Get a human-readable name for the faction
    pub fn name(&self) -> &'static str {
        match self {
            Faction::Mechanists => "Mechanists",
            Faction::Synthetics => "Synthetics",
            Faction::Nomads => "Nomads (Desert Wanderers)",
            Faction::ArcaneEngineers => "Arcane Engineers",
            Faction::CorporateMercenaries => "Corporate Mercenaries",
            Faction::VoidHarbingers => "Void Harbingers",
            Faction::SwarmCollective => "Swarm Collective",
        }
    }
    
    /// Get a description of the faction
    pub fn description(&self) -> &'static str {
        match self {
            Faction::Mechanists => "Masters of industrial machinery and heavy weaponry. Rely on robust mechanical engineering, steam power, and industrial-scale production.",
            Faction::Synthetics => "Pioneers of advanced technology and artificial intelligence. Represent the cutting edge of technological development, focusing on energy weapons, drones, and computational systems.",
            Faction::Nomads => "Masters of mobility and adaptation in harsh environments. Tribal societies that have adapted to harsh desert environments, combining ancient traditions with salvaged technology.",
            Faction::ArcaneEngineers => "Manipulators of energy fields and crystalline technology. Combine advanced technology with mystical energy manipulation, using crystalline structures and energy fields.",
            Faction::CorporateMercenaries => "Profit-driven specialists in modular technology. A private military corporation that specializes in adaptable modular technology and contractual resource extraction.",
            Faction::VoidHarbingers => "Manipulators of gravity and dark energy. Harness dark energy and gravitational manipulation, with alien-inspired technology that defies conventional understanding.",
            Faction::SwarmCollective => "Hive-mind entities with distributed intelligence. Composed of countless small units that can combine and separate as needed, with biological and mechanical elements working in perfect harmony.",
        }
    }
    
    /// Get a representative color for the faction
    pub fn color(&self) -> Color {
        match self {
            Faction::Mechanists => Color::srgb(0.6, 0.6, 0.7),      // Steel blue
            Faction::Synthetics => Color::srgb(0.1, 0.6, 0.8),      // Bright blue
            Faction::Nomads => Color::srgb(0.8, 0.6, 0.2),          // Desert sand
            Faction::ArcaneEngineers => Color::srgb(0.7, 0.3, 0.7),  // Crystal purple
            Faction::CorporateMercenaries => Color::srgb(0.2, 0.5, 0.3), // Corporate green
            Faction::VoidHarbingers => Color::srgb(0.2, 0.0, 0.3),   // Deep void purple
            Faction::SwarmCollective => Color::srgb(0.8, 0.2, 0.2),  // Swarm red
        }
    }
    
    /// Get a key technology for the faction
    pub fn key_technology(&self) -> &'static str {
        match self {
            Faction::Mechanists => "Steam Power, Heavy Armor Plating, Mechanical Walkers",
            Faction::Synthetics => "Energy Weapons, Drone Swarms, AI Tactical Systems",
            Faction::Nomads => "Lightweight Armaments, Mobile Refineries, Desert Spirits",
            Faction::ArcaneEngineers => "Crystal Weaponry, Energy Lattice, Ley Line Tapping",
            Faction::CorporateMercenaries => "Modular Weapons, Market Manipulation, Corporate Espionage",
            Faction::VoidHarbingers => "Void Weapons, Gravity Manipulation, Reality Anchor",
            Faction::SwarmCollective => "Swarm Tactics, Living Architecture, Assimilation",
        }
    }
    
    /// Get all available factions
    pub fn iter() -> impl Iterator<Item = Faction> {
        [
            Faction::Mechanists,
            Faction::Synthetics,
            Faction::Nomads,
            Faction::ArcaneEngineers,
            Faction::CorporateMercenaries,
            Faction::VoidHarbingers,
            Faction::SwarmCollective,
        ].iter().copied()
    }
}
