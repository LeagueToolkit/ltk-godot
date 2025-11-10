//! Texture Import Plugin
//!
//! EditorImportPlugin for League of Legends texture formats (DDS, TEX)

use godot::builtin::{PackedStringArray, Variant};
use godot::classes::{EditorImportPlugin, IEditorImportPlugin};
use godot::global::Error;
use godot::prelude::*;

/// EditorImportPlugin for League of Legends texture files
#[derive(GodotClass)]
#[class(tool, base=EditorImportPlugin)]
pub struct TextureImporter {
    base: Base<EditorImportPlugin>,
}

#[godot_api]
impl IEditorImportPlugin for TextureImporter {
    fn init(base: Base<EditorImportPlugin>) -> Self {
        godot_print!("Initializing TextureImporter");
        Self { base }
    }

    /// Returns the name of the plugin shown in the import menu
    fn get_importer_name(&self) -> GString {
        "ltk.texture".into()
    }

    /// Returns the human-readable name shown in the import inspector
    fn get_visible_name(&self) -> GString {
        "League of Legends Texture".into()
    }

    /// Returns the resource type imported by this plugin
    fn get_resource_type(&self) -> GString {
        "CompressedTexture2D".into()
    }

    /// Returns the list of file extensions recognized by this importer
    fn get_recognized_extensions(&self) -> PackedStringArray {
        let mut extensions = PackedStringArray::new();
        extensions.push("tex");
        extensions
    }

    /// Returns the file extension to use for the imported resource
    fn get_save_extension(&self) -> GString {
        "ctex".into()
    }

    /// Returns the number of presets available for this importer
    fn get_preset_count(&self) -> i32 {
        2
    }

    /// Returns the name of the preset at the given index
    fn get_preset_name(&self, preset_idx: i32) -> GString {
        match preset_idx {
            0 => "2D Texture".into(),
            1 => "3D Texture".into(),
            _ => "Unknown".into(),
        }
    }

    /// Returns the import options for the given preset
    fn get_import_options(&self, _path: GString, preset_idx: i32) -> Array<Dictionary> {
        let mut options = Array::new();

        // Option: Compression mode
        let mut compression_option = Dictionary::new();
        compression_option.set("name", "compress/mode");
        compression_option.set("default_value", 0); // Lossless
        compression_option.set("property_hint", 2); // PROPERTY_HINT_ENUM
        compression_option.set("hint_string", "Lossless,Lossy,Uncompressed");
        options.push(&compression_option);

        // Option: Mipmaps
        let mut mipmaps_option = Dictionary::new();
        mipmaps_option.set("name", "mipmaps/generate");
        mipmaps_option.set("default_value", true);
        mipmaps_option.set("property_hint", 0);
        options.push(&mipmaps_option);

        // Option: SRGB
        let mut srgb_option = Dictionary::new();
        srgb_option.set("name", "process/srgb");
        srgb_option.set("default_value", preset_idx == 0); // True for 2D, false for 3D
        srgb_option.set("property_hint", 0);
        options.push(&srgb_option);

        // Option: Fix Alpha Border
        let mut fix_alpha_option = Dictionary::new();
        fix_alpha_option.set("name", "process/fix_alpha_border");
        fix_alpha_option.set("default_value", true);
        fix_alpha_option.set("property_hint", 0);
        options.push(&fix_alpha_option);

        // Option: Normal Map
        let mut normal_map_option = Dictionary::new();
        normal_map_option.set("name", "process/normal_map");
        normal_map_option.set("default_value", false);
        normal_map_option.set("property_hint", 0);
        options.push(&normal_map_option);

        options
    }

    /// Returns the option visibility for dynamic options display
    fn get_option_visibility(
        &self,
        _path: GString,
        _option_name: StringName,
        _options: Dictionary,
    ) -> bool {
        true
    }

    /// Returns the import order (higher means later)
    fn get_import_order(&self) -> i32 {
        0
    }

    /// Returns the priority of this importer for the given file
    fn get_priority(&self) -> f32 {
        1.0
    }

    /// Performs the actual import
    fn import(
        &self,
        source_file: GString,
        save_path: GString,
        options: Dictionary,
        _platform_variants: Array<GString>,
        _gen_files: Array<GString>,
    ) -> Error {
        godot_print!("Importing texture file: {}", source_file);
        godot_print!("Save path: {}", save_path);

        // Extract import options
        let compress_mode = options
            .get("compress/mode")
            .unwrap_or(Variant::from(0))
            .to::<i32>();
        let generate_mipmaps = options
            .get("mipmaps/generate")
            .unwrap_or(Variant::from(true))
            .to::<bool>();
        let srgb = options
            .get("process/srgb")
            .unwrap_or(Variant::from(true))
            .to::<bool>();
        let fix_alpha_border = options
            .get("process/fix_alpha_border")
            .unwrap_or(Variant::from(true))
            .to::<bool>();
        let normal_map = options
            .get("process/normal_map")
            .unwrap_or(Variant::from(false))
            .to::<bool>();

        godot_print!(
            "Import options - compress: {}, mipmaps: {}, srgb: {}, fix_alpha: {}, normal_map: {}",
            compress_mode,
            generate_mipmaps,
            srgb,
            fix_alpha_border,
            normal_map
        );

        // TODO: Implement actual texture loading and conversion using league-toolkit
        // For now, this is a placeholder that returns OK
        //
        // Steps to implement:
        // 1. Read the source file using league-toolkit's texture parsing
        // 2. Convert the texture data to Godot's Image format
        // 3. Apply mipmaps, compression, and other processing if requested
        // 4. Save the resulting texture to save_path using ResourceSaver

        godot_warn!("TextureImporter: Actual import not yet implemented");

        // Return OK for success, or ERR_* constant for errors
        Error::OK
    }
}
