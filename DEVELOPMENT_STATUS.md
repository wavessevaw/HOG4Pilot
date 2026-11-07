# Development status — 2026-11-07

Not version 1.0. Core and visual-control stages remain incomplete.

Implemented: prior Rust engine retained; SQLite current project plus historical revisions; validated saved looks; Tauri Rust commands and React editor source; synthetic stage display from backend frames; CSV importer for RGB/intensity/locked channels; Windows runner workflow producing NSIS artifacts when successful.

Validated: 13 Rust tests pass; React production build passes. Windows/Tauri desktop build NOT confirmed. No remote repository or GitHub CI run is available. No physical output was used.

Limitations: editor controls the entire virtual rig, not individual zones; stored transition durations are not scheduled; three presets only; CSV parser not exposed in UI; SQLite does not yet persist venues/profiles. No discovery, RDM, sACN, Hog, OCR, watchdog, output process, engineering verification UI. No automatic project save yet. No UI tests yet.

Next concrete task: install/check desktop prerequisites, compile src-tauri, fix IPC/configuration issues, then add backend timed playback with tests and browser integration tests. Keep physical output disabled.

## Venue Auto-Mapping continuation

Added 24-fixture synthetic venue, step answers with Pending/Unknown/Known states, passive hazer and unverified fixtures, per-fixture synthetic identification, observed coverage zones, grouping, warm-look frame generation, SQLite resume, React wizard. No physical discovery/RDM or Pan/Tilt calibration is claimed. Identification and playback only return simulated frames; no hardware network output exposed. Sixteen Rust tests and frontend build passed before publication; Windows result pending.
