//! File format loaders for League of Legends assets
//!
//! This module contains loaders for various League of Legends file formats.

pub mod wad_loader;
pub mod mesh_loader;
pub mod texture_loader;

pub use wad_loader::WadLoader;
pub use mesh_loader::MeshLoader;
pub use texture_loader::TextureLoader;
