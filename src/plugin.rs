// This file is intentionally left empty as the dynamic plugin loading system
// has been removed or is no longer used for menus.

/// Plugin type identifier for dynamic loading
#[repr(C)]
pub enum PluginType {
    /// Regular game plugin
    Game,
    /// Menu plugin
    Menu,
}

/// Plugin factory function type for dynamic loading
pub type CreatePluginFn = extern "C" fn() -> *mut sf_plugin_template::PluginHandle;

