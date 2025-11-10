//! # ltk-godot
//!
//! Godot 4 extension for importing League of Legends file formats using league-toolkit.
//!
//! This extension provides loaders and importers for various League of Legends file formats
//! including WAD archives, meshes, textures, animations, and metadata.

use godot::prelude::*;

mod loaders;
mod importers;
mod plugin;

// Re-export loaders for easy access
pub use loaders::{MeshLoader, TextureLoader, WadLoader};

// Re-export importers
pub use importers::{MeshImporter, TextureImporter};

// Re-export plugin
pub use plugin::LtkEditorPlugin;

struct LtkGodotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for LtkGodotExtension {}
