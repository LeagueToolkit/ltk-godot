# ltk-godot

Godot 4 extension for importing League of Legends file formats using [league-toolkit](https://github.com/LeagueToolkit/league-toolkit).

## Overview

`ltk-godot` is a GDExtension written in Rust that brings League of Legends asset importing capabilities to Godot 4. It leverages the powerful `league-toolkit` library to parse and load various LoL file formats.

## Supported Formats

- **WAD Archives** - Container format for League of Legends assets
- **Meshes** - 3D model formats (SKN, SCB, SCO)
- **Textures** - Texture formats (DDS, TEX)
- **Animations** - Animation data (ANM)
- **Metadata** - Game metadata (BIN)

## Building

### Prerequisites

- Rust 1.70 or later
- Cargo
- Godot 4.1 or later (for testing)

### Build Steps

```bash
# Clone the repository
git clone https://github.com/LeagueToolkit/ltk-godot.git
cd ltk-godot

# Build the extension (debug)
cargo build

# Build the extension (release)
cargo build --release
```

The compiled library will be placed in `target/debug/` or `target/release/` depending on your build configuration.

## Usage in Godot

1. Copy the entire `ltk-godot` directory into your Godot project
2. The `ltk_godot.gdextension` file tells Godot where to find the native library
3. The extension classes will be available in GDScript:

```gdscript
# Example: Loading a WAD archive
var wad_loader = WadLoader.new()
wad_loader.load("res://path/to/archive.wad")
var entry_count = wad_loader.get_entry_count()
print("WAD contains ", entry_count, " entries")
```

## Development

This project uses:
- [godot-rust (gdext)](https://github.com/godot-rust/gdext) - Rust bindings for Godot 4
- [league-toolkit](https://github.com/LeagueToolkit/league-toolkit) - Rust library for League of Legends file formats

## License

This project is licensed under the GNU Affero General Public License v3.0 (AGPL-3.0) - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
