use bevy::prelude::*;
use thiserror::Error;
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
            name: GamePlugin::name(self),
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

/// Error type for plugin loading
#[derive(Error, Debug)]
pub enum PluginLoadError {
    /// Failed to load the plugin library
    #[error("Failed to load plugin library: {0}")]
    LibraryLoad(#[from] libloading::Error),
    
    /// Failed to get plugin symbol
    #[error("Failed to get plugin symbol: {0}")]
    SymbolError(String),
    
    /// Plugin initialization failed
    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),
}

/// Load a plugin from a dynamic library
pub fn load_plugin(path: impl AsRef<std::path::Path>) -> Result<Box<dyn GamePlugin>, PluginLoadError> {
    // Implementation placeholder
    Err(PluginLoadError::SymbolError("Not implemented".to_string()))
}
