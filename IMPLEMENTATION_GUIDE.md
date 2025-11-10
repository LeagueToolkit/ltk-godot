# Implementation Guide

This guide is for developers who want to implement the actual file parsing and conversion logic for the EditorImportPlugins.

## Current Status

The EditorImportPlugin infrastructure is complete and functional. The plugins are registered with Godot and will be called when the appropriate file types are added to a project. However, the actual file parsing and conversion is not yet implemented - the `import()` methods currently return `Error::OK` without doing any actual work.

## What Needs to be Implemented

### 1. Mesh Importer (`src/importers/mesh_importer.rs`)

The `import()` method needs to:

1. **Read the source file**
   ```rust
   // Use league-toolkit to parse the mesh file
   use league_toolkit::mesh::{Skn, Scb, Sco};
   
   // Determine file type from extension
   let extension = source_file.to_string().split('.').last().unwrap();
   let mesh_data = match extension {
       "skn" => parse_skn_file(&source_file),
       "scb" => parse_scb_file(&source_file),
       "sco" => parse_sco_file(&source_file),
       _ => return Error::ERR_FILE_UNRECOGNIZED,
   };
   ```

2. **Convert to Godot's ArrayMesh format**
   ```rust
   use godot::classes::{ArrayMesh, Mesh as GodotMesh};
   use godot::builtin::VariantArray;
   
   // Create ArrayMesh
   let mut array_mesh = ArrayMesh::new_gd();
   
   // Convert mesh data to Godot arrays
   let mut arrays = VariantArray::new();
   arrays.resize(ArrayMesh::ARRAY_MAX as usize);
   
   // Set vertex positions (ARRAY_VERTEX = 0)
   let vertices = PackedVector3Array::from(&mesh_data.vertices);
   arrays.set(ArrayMesh::ARRAY_VERTEX as usize, vertices.to_variant());
   
   // Set normals (ARRAY_NORMAL = 1)
   if mesh_data.has_normals || generate_normals {
       let normals = calculate_or_use_normals(&mesh_data, generate_normals);
       arrays.set(ArrayMesh::ARRAY_NORMAL as usize, normals.to_variant());
   }
   
   // Set UVs (ARRAY_TEX_UV = 4)
   let uvs = PackedVector2Array::from(&mesh_data.uvs);
   arrays.set(ArrayMesh::ARRAY_TEX_UV as usize, uvs.to_variant());
   
   // Set indices (ARRAY_INDEX = 12)
   let indices = PackedInt32Array::from(&mesh_data.indices);
   arrays.set(ArrayMesh::ARRAY_INDEX as usize, indices.to_variant());
   
   // Add surface to mesh
   array_mesh.add_surface_from_arrays(
       godot::classes::mesh::PrimitiveType::TRIANGLES,
       arrays,
       VariantArray::new(),
       Dictionary::new(),
       0
   );
   ```

3. **Apply import options**
   ```rust
   // Apply scale
   if scale != 1.0 {
       // Transform all vertices by scale factor
   }
   
   // Generate tangents if requested
   if generate_tangents {
       array_mesh.surface_generate_tangents(0);
   }
   ```

4. **Save the resource**
   ```rust
   use godot::classes::ResourceSaver;
   
   let mut resource_saver = ResourceSaver::singleton();
   let save_result = resource_saver.save(
       array_mesh.upcast(),
       format!("{}.{}", save_path, get_save_extension()).into()
   );
   
   if save_result != Error::OK {
       godot_error!("Failed to save mesh resource: {:?}", save_result);
       return save_result;
   }
   ```

### 2. Texture Importer (`src/importers/texture_importer.rs`)

The `import()` method needs to:

1. **Read and parse the texture file**
   ```rust
   use league_toolkit::texture::Tex;
   
   // Parse the .tex file
   let tex_data = match Tex::from_file(&source_file.to_string()) {
       Ok(tex) => tex,
       Err(e) => {
           godot_error!("Failed to parse texture: {:?}", e);
           return Error::ERR_FILE_CORRUPT;
       }
   };
   ```

2. **Convert to Godot Image**
   ```rust
   use godot::classes::{Image, ImageTexture};
   
   // Create a Godot Image from the texture data
   let mut image = Image::new();
   
   // Determine format and create image
   let format = match tex_data.format {
       // Map League texture formats to Godot formats
       TexFormat::DXT1 => Image::FORMAT_DXT1,
       TexFormat::DXT5 => Image::FORMAT_DXT5,
       TexFormat::RGBA8 => Image::FORMAT_RGBA8,
       // ... handle other formats
   };
   
   image.create_from_data(
       tex_data.width as i32,
       tex_data.height as i32,
       tex_data.has_mipmaps,
       format,
       tex_data.pixel_data.into()
   );
   ```

3. **Apply import settings**
   ```rust
   // Apply sRGB
   if srgb {
       image.srgb_to_linear();
   }
   
   // Generate mipmaps if needed
   if generate_mipmaps && !image.has_mipmaps() {
       image.generate_mipmaps();
   }
   
   // Fix alpha border
   if fix_alpha_border {
       image.fix_alpha_edges();
   }
   
   // Apply compression
   match compress_mode {
       0 => image.compress(Image::COMPRESS_BPTC), // Lossless
       1 => image.compress(Image::COMPRESS_S3TC), // Lossy
       2 => {}, // Uncompressed
       _ => {}
   }
   ```

4. **Save the texture resource**
   ```rust
   use godot::classes::ResourceSaver;
   
   let mut resource_saver = ResourceSaver::singleton();
   let save_result = resource_saver.save(
       image.upcast(),
       format!("{}.{}", save_path, get_save_extension()).into()
   );
   ```

## Key Godot-Rust Concepts

### Working with Godot File Paths

Godot uses `res://` paths internally. When reading files in the import plugin:

```rust
use godot::classes::{FileAccess, DirAccess};

// Convert res:// path to absolute path if needed
let mut file = FileAccess::open(source_file.clone(), FileAccess::READ);
let bytes = file.get_buffer(file.get_length() as i32);
```

### Error Handling

Always return appropriate `Error` enum values:
- `Error::OK` - Success
- `Error::ERR_FILE_NOT_FOUND` - File doesn't exist
- `Error::ERR_FILE_CORRUPT` - File is invalid/corrupted
- `Error::ERR_FILE_UNRECOGNIZED` - Unknown format
- `Error::FAILED` - Generic failure

### Type Conversions

Converting Rust data to Godot types:

```rust
// Rust Vec to PackedArray
let rust_vec: Vec<Vector3> = vec![...];
let godot_array = PackedVector3Array::from(&rust_vec);

// Godot types to Variant
let variant = godot_array.to_variant();
```

## Testing

To test your implementation:

1. Build the extension: `cargo build`
2. Open a Godot project with the extension
3. Enable the plugin in Project Settings
4. Drop a League file into the project
5. Check the console for debug output
6. Verify the imported resource in the inspector

## Debugging Tips

1. Use `godot_print!()`, `godot_warn!()`, and `godot_error!()` for logging
2. Check the Godot console for output
3. Use `RUST_BACKTRACE=1` when running Godot from terminal
4. Test with small, simple files first
5. Validate that league-toolkit is parsing files correctly before converting to Godot

## Resources

- [Godot-Rust Book](https://godot-rust.github.io/book/)
- [godot-rust API Docs](https://godot-rust.github.io/docs/gdext/)
- [Godot ArrayMesh Documentation](https://docs.godotengine.org/en/stable/classes/class_arraymesh.html)
- [Godot Image Documentation](https://docs.godotengine.org/en/stable/classes/class_image.html)
- [League Toolkit Documentation](https://github.com/LeagueToolkit/league-toolkit)

## Next Steps

1. Start with the texture importer as it's generally simpler
2. Implement basic file reading and parsing
3. Convert to Godot Image format
4. Test with various texture files
5. Move on to mesh importer
6. Implement vertex/normal/UV conversion
7. Test with various mesh files
8. Add error handling and edge cases
9. Optimize performance for large files

