# DisplayFlow Studio

GUI application for display management, live wallpaper, and animation editing.

## Dependencies

This project depends on the following crates from the [displaymanager](https://github.com/piot5/displaymanager) workspace:
- `df_ddc` - DDC/CI monitor control
- `df_displmgr` - Cross-platform display configuration
- `df_displmgr_info` - EDID parsing and display information

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

For wallpaper mode:
```bash
cargo run -- wallpaper /path/to/package.zip