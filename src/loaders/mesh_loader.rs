//! Mesh Loader
//!
//! Provides functionality to load League of Legends mesh formats (SKN, SCB, SCO).

use godot::prelude::*;

/// Loader for League of Legends mesh files
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct MeshLoader {
    base: Base<RefCounted>,
    path: GString,
    vertex_count: i32,
}

#[godot_api]
impl IRefCounted for MeshLoader {
    fn init(base: Base<RefCounted>) -> Self {
        MeshLoader {
            base,
            path: GString::new(),
            vertex_count: 0,
        }
    }
}

#[godot_api]
impl MeshLoader {
    /// Load a mesh file from the specified path
    /// Supports .skn, .scb, and .sco formats
    #[func]
    pub fn load(&mut self, path: GString) -> bool {
        self.path = path.clone();
        godot_print!("Loading mesh from: {}", path);
        // TODO: Implement actual mesh loading using league-toolkit's ltk_mesh
        true
    }

    /// Get the path of the currently loaded mesh
    #[func]
    pub fn get_path(&self) -> GString {
        self.path.clone()
    }

    /// Get the number of vertices in the mesh
    #[func]
    pub fn get_vertex_count(&self) -> i32 {
        self.vertex_count
    }

    /// Get mesh bounds (returns a dictionary with min and max Vector3)
    #[func]
    pub fn get_bounds(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        // TODO: Calculate actual bounds from loaded mesh data
        dict.set("min", Vector3::new(0.0, 0.0, 0.0));
        dict.set("max", Vector3::new(0.0, 0.0, 0.0));
        dict
    }
}
