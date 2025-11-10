# Examples

This directory would contain example Godot projects demonstrating how to use ltk-godot.

## Basic Usage Example

Here's a simple example of using the extension in GDScript:

```gdscript
extends Node

func _ready():
    # Load a WAD archive
    var wad = WadLoader.new()
    if wad.load("res://assets/league/Data.wad"):
        print("WAD loaded successfully!")
        print("Entry count: ", wad.get_entry_count())
    
    # Load a mesh
    var mesh = MeshLoader.new()
    if mesh.load("res://assets/league/model.skn"):
        print("Mesh loaded successfully!")
        print("Vertices: ", mesh.get_vertex_count())
        var bounds = mesh.get_bounds()
        print("Bounds min: ", bounds["min"])
        print("Bounds max: ", bounds["max"])
    
    # Load a texture
    var texture = TextureLoader.new()
    if texture.load("res://assets/league/texture.dds"):
        print("Texture loaded successfully!")
        print("Dimensions: ", texture.get_dimensions())
```

## Integration Example

Project structure when using ltk-godot:

```
my_godot_project/
├── addons/
│   └── ltk-godot/
│       ├── ltk_godot.gdextension
│       ├── target/
│       │   ├── debug/
│       │   │   └── libltk_godot.so
│       │   └── release/
│       │       └── libltk_godot.so
│       └── ...
├── assets/
│   └── league/
│       ├── Data.wad
│       ├── model.skn
│       └── texture.dds
├── scenes/
│   └── main.tscn
└── project.godot
```

## Advanced Usage

### Custom Resource Loader

You can create custom resource loaders that use ltk-godot to automatically import League of Legends assets:

```gdscript
@tool
extends EditorImportPlugin

func _get_importer_name():
    return "league.mesh"

func _get_visible_name():
    return "League Mesh"

func _get_recognized_extensions():
    return ["skn", "scb", "sco"]

func _get_resource_type():
    return "Mesh"

func _import(source_file, save_path, options, r_platform_variants, r_gen_files):
    var loader = MeshLoader.new()
    if loader.load(source_file):
        # Convert to Godot mesh format
        # Save as .res or .tres
        return OK
    return FAILED
```

This allows you to drag-and-drop League of Legends files directly into your Godot project!
