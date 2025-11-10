# Contributing to ltk-godot

Thank you for your interest in contributing to ltk-godot!

## Development Setup

1. **Install Rust**: Make sure you have Rust 1.70+ installed
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**:
   ```bash
   git clone https://github.com/LeagueToolkit/ltk-godot.git
   cd ltk-godot
   ```

3. **Build the project**:
   ```bash
   cargo build
   ```

## Project Structure

```
ltk-godot/
├── src/
│   ├── lib.rs              # Main library entry point
│   └── loaders/            # File format loaders
│       ├── mod.rs
│       ├── wad_loader.rs
│       ├── mesh_loader.rs
│       └── texture_loader.rs
├── ltk_godot.gdextension   # Godot extension configuration
├── Cargo.toml              # Rust dependencies
└── README.md
```

## Adding New Features

### Adding a New Loader

1. Create a new file in `src/loaders/` (e.g., `animation_loader.rs`)
2. Implement the loader using `godot::prelude::*` and the `#[derive(GodotClass)]` macro
3. Add the module to `src/loaders/mod.rs`
4. Export the loader from `src/lib.rs`
5. Update documentation

### Example Loader Template

```rust
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct MyLoader {
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for MyLoader {
    fn init(base: Base<RefCounted>) -> Self {
        MyLoader { base }
    }
}

#[godot_api]
impl MyLoader {
    #[func]
    pub fn load(&mut self, path: GString) -> bool {
        // Implementation
        true
    }
}
```

## Testing

Run tests with:
```bash
cargo test
```

## Code Style

- Follow Rust conventions and use `cargo fmt` to format code
- Run `cargo clippy` to check for common mistakes
- Write clear documentation comments for public APIs

## Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Make your changes
4. Run tests and ensure code compiles
5. Commit your changes (`git commit -am 'Add new feature'`)
6. Push to the branch (`git push origin feature/my-feature`)
7. Open a Pull Request

## License

By contributing, you agree that your contributions will be licensed under the AGPL-3.0 license.
