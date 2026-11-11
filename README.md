# displaymanager_studio

Rust-GUI-Anwendung zur Displayverwaltung – Teil des **DisplayFlow**-Ökosystems.

Dies ist das grafische Frontend zum [`displaymanager`](https://github.com/piot5/displaymanager)-Toolkit.
Es baut auf `df_displmgr` (Display-Topologie) und `df_displmgr_info` (EDID/Telemetrie) auf.

## Status

Scaffold / Proof-of-Concept. Windows-GUI (wgpu/egui + winit) ist noch nicht mit dem
Display-Toolkit verbunden. Nach einer Publikation von `df_core`/`df_displmgr`
kann das Display-Listing und -Steuerung realisiert werden.

## Arbeiten

```bash
cargo check --bins --lib
```
