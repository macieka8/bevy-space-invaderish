## Release build
- in Cargo.toml remove `features = ["dynamic_linking"]` from bevy
- in .cargo/Cargo.toml add `"-Ctarget-feature=+crt-static"` to `target.x86_64-pc-windows-msvc` (not necessary)
- run `cargo build -r`
- copy asset folder to executable location

## Game Controls
- `A, D` - movement
- `J` - shoot
- `F3` - draw gizmos
- `Esc` - quit game
