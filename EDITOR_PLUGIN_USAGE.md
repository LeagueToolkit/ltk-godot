# Editor Import Plugin Usage

This guide explains how to use the LeagueToolkit Editor Import Plugin in Godot 4.

## What is an EditorImportPlugin?

An `EditorImportPlugin` allows Godot to automatically recognize and import custom file formats when you drag them into your project. Unlike manual loaders, the import plugin integrates seamlessly with Godot's asset pipeline.

## Installation

1. Copy the entire `ltk-godot` directory into your Godot project
2. The `ltk_godot.gdextension` file tells Godot where to find the native library
3. Ensure you have built the extension (see the main README for build instructions)

## Enabling the Plugin

To enable the import plugin in the Godot editor:

1. Open your project in Godot 4
2. Go to `Project > Project Settings > Plugins`
3. Find "LeagueToolkit Importer" in the list
4. Check the box to enable it

You should see console output indicating the plugin has been loaded and import plugins registered.

## Supported File Formats

### Mesh Files
- **Extensions**: `.skn`, `.scb`, `.sco`
- **Imports as**: Godot Mesh resource
- **Import Options**:
  - `scale`: Scale factor for the mesh (default: 1.0)
  - `generate_normals`: Generate normals if not present (default: false)
  - `generate_tangents`: Generate tangents for normal mapping (default: true)

### Texture Files
- **Extensions**: `.tex`
- **Imports as**: CompressedTexture2D
- **Import Presets**:
  - **2D Texture**: Default preset for 2D textures with sRGB enabled
  - **3D Texture**: Preset for 3D textures with sRGB disabled
- **Import Options**:
  - `compress/mode`: Compression mode (Lossless, Lossy, or Uncompressed)
  - `mipmaps/generate`: Generate mipmaps for LOD (default: true)
  - `process/srgb`: Use sRGB color space (default: true for 2D preset)
  - `process/fix_alpha_border`: Fix alpha borders to prevent seams (default: true)
  - `process/normal_map`: Treat as normal map (default: false)

## Using Imported Assets

### Automatic Import

Simply drag and drop your League of Legends asset files into your Godot project:

1. Open the FileSystem panel in Godot
2. Drag `.skn`, `.scb`, `.sco`, or `.tex` files into your project
3. Godot will automatically detect and import them using the appropriate plugin
4. The imported resources will have a `.import` file generated alongside them

### Configuring Import Settings

1. Select the imported file in the FileSystem panel
2. The Import dock will appear (usually on the left side)
3. Adjust the import options as needed
4. Click "Reimport" to apply the changes

### Using in Scenes

Once imported, you can use the resources just like any other Godot asset:

```gdscript
# For meshes
var mesh_instance = MeshInstance3D.new()
mesh_instance.mesh = load("res://path/to/your/model.skn")
add_child(mesh_instance)

# For textures
var sprite = Sprite2D.new()
sprite.texture = load("res://path/to/your/texture.tex")
add_child(sprite)
```

## Current Implementation Status

⚠️ **Note**: The import plugins are currently set up with placeholder implementations. The actual file parsing and conversion using `league-toolkit` needs to be implemented.

**TODO for full functionality**:
- Implement mesh parsing using `league-toolkit`'s mesh module
- Convert League mesh data to Godot's `ArrayMesh` format
- Implement texture parsing using `league-toolkit`'s texture module
- Convert League texture data to Godot's `Image` format
- Add proper error handling and validation
- Implement resource saving using Godot's `ResourceSaver`

## Troubleshooting

### Plugin not appearing in the Plugins list
- Ensure the extension is built (`cargo build` or `cargo build --release`)
- Check that the `.gdextension` file is in the project root
- Verify that the library path in `.gdextension` matches your build output

### Import fails with errors
- Check the Godot console for error messages
- Ensure the source file is a valid League of Legends asset
- Try adjusting import settings

### Changes not taking effect
- Make sure to click "Reimport" after changing settings
- Try cleaning the `.godot/imported` directory and reimporting

## Manual Loaders (Alternative)

If you prefer manual control over loading, the extension also provides loader classes:

```gdscript
# Manual mesh loading
var mesh_loader = MeshLoader.new()
mesh_loader.load("res://path/to/model.skn")

# Manual texture loading
var texture_loader = TextureLoader.new()
texture_loader.load("res://path/to/texture.tex")
```

However, using the EditorImportPlugin provides better integration with Godot's workflow.

