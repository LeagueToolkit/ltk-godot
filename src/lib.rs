//! # ltk-godot
//!
//! Godot 4 extension for importing League of Legends file formats using league-toolkit.
//!
//! This extension provides loaders and importers for various League of Legends file formats
//! including WAD archives, meshes, textures, animations, and metadata.

use godot::prelude::*;

mod loaders;

// Re-export loaders for easy access
pub use loaders::{MeshLoader, TextureLoader, WadLoader};

struct LtkGodotExtension;

#[gdextension]
unsafe impl ExtensionLibrary for LtkGodotExtension {}
