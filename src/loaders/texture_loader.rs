//! Texture Loader
//!
//! Provides functionality to load League of Legends texture formats (DDS, TEX).

use godot::prelude::*;

/// Loader for League of Legends texture files
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct TextureLoader {
    base: Base<RefCounted>,
    path: GString,
    width: i32,
    height: i32,
}

#[godot_api]
impl IRefCounted for TextureLoader {
    fn init(base: Base<RefCounted>) -> Self {
        TextureLoader {
            base,
            path: GString::new(),
            width: 0,
            height: 0,
        }
    }
}

#[godot_api]
impl TextureLoader {
    /// Load a texture file from the specified path
    /// Supports .dds and .tex formats
    #[func]
    pub fn load(&mut self, path: GString) -> bool {
        self.path = path.clone();
        godot_print!("Loading texture from: {}", path);
        // TODO: Implement actual texture loading using league-toolkit's ltk_texture
        true
    }

    /// Get the path of the currently loaded texture
    #[func]
    pub fn get_path(&self) -> GString {
        self.path.clone()
    }

    /// Get the width of the texture
    #[func]
    pub fn get_width(&self) -> i32 {
        self.width
    }

    /// Get the height of the texture
    #[func]
    pub fn get_height(&self) -> i32 {
        self.height
    }

    /// Get the texture dimensions as a Vector2
    #[func]
    pub fn get_dimensions(&self) -> Vector2 {
        Vector2::new(self.width as f32, self.height as f32)
    }
}
