//! Mesh Import Plugin
//!
//! EditorImportPlugin for League of Legends mesh formats (SKN, SCB, SCO)

use godot::prelude::*;
use godot::classes::{EditorImportPlugin, IEditorImportPlugin};
use godot::builtin::{PackedStringArray, Variant};
use godot::global::Error;

/// EditorImportPlugin for League of Legends mesh files
#[derive(GodotClass)]
#[class(tool, base=EditorImportPlugin)]
pub struct MeshImporter {
    base: Base<EditorImportPlugin>,
}

#[godot_api]
impl IEditorImportPlugin for MeshImporter {
    fn init(base: Base<EditorImportPlugin>) -> Self {
        godot_print!("Initializing MeshImporter");
        Self { base }
    }

    /// Returns the name of the plugin shown in the import menu
    fn get_importer_name(&self) -> GString {
        "ltk.mesh".into()
    }

    /// Returns the human-readable name shown in the import inspector
    fn get_visible_name(&self) -> GString {
        "League of Legends Mesh".into()
    }

    /// Returns the resource type imported by this plugin
    fn get_resource_type(&self) -> GString {
        "Mesh".into()
    }

    /// Returns the list of file extensions recognized by this importer
    fn get_recognized_extensions(&self) -> PackedStringArray {
        let mut extensions = PackedStringArray::new();
        extensions.push("skn");
        extensions.push("scb");
        extensions.push("sco");
        extensions
    }

    /// Returns the file extension to use for the imported resource
    fn get_save_extension(&self) -> GString {
        "mesh".into()
    }

    /// Returns the number of presets available for this importer
    fn get_preset_count(&self) -> i32 {
        1
    }

    /// Returns the name of the preset at the given index
    fn get_preset_name(&self, preset_idx: i32) -> GString {
        match preset_idx {
            0 => "Default".into(),
            _ => "Unknown".into(),
        }
    }

    /// Returns the import options for the given preset
    fn get_import_options(&self, _path: GString, _preset_idx: i32) -> Array<Dictionary> {
        let mut options = Array::new();

        // Option: Scale
        let mut scale_option = Dictionary::new();
        scale_option.set("name", "scale");
        scale_option.set("default_value", 1.0);
        scale_option.set("property_hint", 0); // PROPERTY_HINT_NONE
        options.push(&scale_option);

        // Option: Generate Normals
        let mut normals_option = Dictionary::new();
        normals_option.set("name", "generate_normals");
        normals_option.set("default_value", false);
        normals_option.set("property_hint", 0);
        options.push(&normals_option);

        // Option: Generate Tangents
        let mut tangents_option = Dictionary::new();
        tangents_option.set("name", "generate_tangents");
        tangents_option.set("default_value", true);
        tangents_option.set("property_hint", 0);
        options.push(&tangents_option);

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
        godot_print!("Importing mesh file: {}", source_file);
        godot_print!("Save path: {}", save_path);
        
        // Extract import options
        let scale = options.get("scale").unwrap_or(Variant::from(1.0)).to::<f64>();
        let generate_normals = options.get("generate_normals").unwrap_or(Variant::from(false)).to::<bool>();
        let generate_tangents = options.get("generate_tangents").unwrap_or(Variant::from(true)).to::<bool>();

        godot_print!("Import options - scale: {}, normals: {}, tangents: {}", 
                    scale, generate_normals, generate_tangents);

        // TODO: Implement actual mesh loading and conversion using league-toolkit
        // For now, this is a placeholder that returns OK
        // 
        // Steps to implement:
        // 1. Read the source file using league-toolkit's mesh parsing
        // 2. Convert the mesh data to Godot's ArrayMesh format
        // 3. Apply scale, generate normals/tangents if requested
        // 4. Save the resulting mesh to save_path using ResourceSaver

        godot_warn!("MeshImporter: Actual import not yet implemented");
        
        // Return OK for success, or ERR_* constant for errors
        Error::OK
    }
}

