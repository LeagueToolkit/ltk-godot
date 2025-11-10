//! WAD Archive Loader
//!
//! Provides functionality to load and access WAD archives used by League of Legends.

use godot::prelude::*;

/// Loader for League of Legends WAD archive files
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct WadLoader {
    base: Base<RefCounted>,
    path: GString,
}

#[godot_api]
impl IRefCounted for WadLoader {
    fn init(base: Base<RefCounted>) -> Self {
        WadLoader {
            base,
            path: GString::new(),
        }
    }
}

#[godot_api]
impl WadLoader {
    /// Load a WAD archive from the specified path
    #[func]
    pub fn load(&mut self, path: GString) -> bool {
        self.path = path.clone();
        godot_print!("Loading WAD archive from: {}", path);
        // TODO: Implement actual WAD loading using league-toolkit
        true
    }

    /// Get the path of the currently loaded WAD archive
    #[func]
    pub fn get_path(&self) -> GString {
        self.path.clone()
    }

    /// Get the number of entries in the WAD archive
    #[func]
    pub fn get_entry_count(&self) -> i32 {
        // TODO: Implement using league-toolkit
        0
    }
}
