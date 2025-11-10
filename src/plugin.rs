//! Editor Plugin
//!
//! Main EditorPlugin that registers all import plugins for League of Legends file formats

use godot::prelude::*;
use godot::classes::{EditorPlugin, IEditorPlugin, EditorImportPlugin};
use crate::importers::{MeshImporter, TextureImporter};

/// Main editor plugin for ltk-godot
/// Registers all import plugins when enabled
#[derive(GodotClass)]
#[class(tool, base=EditorPlugin, init)]
pub struct LtkEditorPlugin {
    base: Base<EditorPlugin>,
    mesh_importer: Option<Gd<MeshImporter>>,
    texture_importer: Option<Gd<TextureImporter>>,
}

#[godot_api]
impl IEditorPlugin for LtkEditorPlugin {
    /// Called when the plugin enters the editor scene tree
    fn enter_tree(&mut self) {
        godot_print!("LtkEditorPlugin: Entering tree");
        
        // Create and add mesh importer
        let mesh_importer = MeshImporter::new_gd();
        self.base_mut().add_import_plugin(&mesh_importer.clone().upcast::<EditorImportPlugin>());
        self.mesh_importer = Some(mesh_importer);
        godot_print!("LtkEditorPlugin: Registered MeshImporter");
        
        // Create and add texture importer
        let texture_importer = TextureImporter::new_gd();
        self.base_mut().add_import_plugin(&texture_importer.clone().upcast::<EditorImportPlugin>());
        self.texture_importer = Some(texture_importer);
        godot_print!("LtkEditorPlugin: Registered TextureImporter");
        
        godot_print!("LtkEditorPlugin: All importers registered successfully");
    }

    /// Called when the plugin exits the editor scene tree
    fn exit_tree(&mut self) {
        godot_print!("LtkEditorPlugin: Exiting tree");
        
        // Remove mesh importer
        if let Some(importer) = self.mesh_importer.take() {
            self.base_mut().remove_import_plugin(&importer.upcast::<EditorImportPlugin>());
            godot_print!("LtkEditorPlugin: Removed MeshImporter");
        }
        
        // Remove texture importer
        if let Some(importer) = self.texture_importer.take() {
            self.base_mut().remove_import_plugin(&importer.upcast::<EditorImportPlugin>());
            godot_print!("LtkEditorPlugin: Removed TextureImporter");
        }
        
        godot_print!("LtkEditorPlugin: All importers removed");
    }

    /// Returns the name of the plugin
    fn get_plugin_name(&self) -> GString {
        "LeagueToolkit Importer".into()
    }
}

