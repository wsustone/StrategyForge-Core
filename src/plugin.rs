use bevy::prelude::*;
use dyn_clone::DynClone;

/// Trait for plugins that can be loaded dynamically
pub trait GamePlugin: Plugin + DynClone + Send + Sync + 'static {
    /// Get plugin name for debugging and identification
    fn name(&self) -> &'static str;
    
    /// Get plugin version
    fn version(&self) -> &'static str;
    
    /// Get plugin metadata
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            name: self.name(),
            version: self.version(),
        }
    }
}

dyn_clone::clone_trait_object!(GamePlugin);

/// Metadata about a loaded plugin
#[derive(Debug, Clone)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: &'static str,
    /// Plugin version
    pub version: &'static str,
}
