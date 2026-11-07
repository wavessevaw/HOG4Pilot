# Development status — 2026-11-07

Not version 1.0. Core and visual-control stages remain incomplete.

Implemented: prior Rust engine retained; SQLite current project plus historical revisions; validated saved looks; Tauri Rust commands and React editor source; synthetic stage display from backend frames; CSV importer for RGB/intensity/locked channels; Windows runner workflow producing NSIS artifacts when successful.

Validated: 13 Rust tests pass; React production build passes. Windows/Tauri desktop build NOT confirmed. No remote repository or GitHub CI run is available. No physical output was used.

Limitations: editor controls the entire virtual rig, not individual zones; stored transition durations are not scheduled; three presets only; CSV parser not exposed in UI; SQLite does not yet persist venues/profiles. No discovery, RDM, sACN, Hog, OCR, watchdog, output process, engineering verification UI. No automatic project save yet. No UI tests yet.

Next concrete task: install/check desktop prerequisites, compile src-tauri, fix IPC/configuration issues, then add backend timed playback with tests and browser integration tests. Keep physical output disabled.

## Venue Auto-Mapping continuation

Added 24-fixture synthetic venue, step answers with Pending/Unknown/Known states, passive hazer and unverified fixtures, per-fixture synthetic identification, observed coverage zones, grouping, warm-look frame generation, SQLite resume, React wizard. No physical discovery/RDM or Pan/Tilt calibration is claimed. Identification and playback only return simulated frames; no hardware network output exposed. Sixteen Rust tests and frontend build passed before publication; Windows result pending.

Publication: pushed to https://github.com/wavessevaw/HOG4Pilot main, preserving original initial commit. Windows CI run https://github.com/wavessevaw/HOG4Pilot/actions/runs/42347148551 started; last observed status in_progress. Installer not confirmed. Previous statements that no remote exists are superseded.

## Patch import and Windows repair

Initial Windows runs failed because src-tauri/icons/icon.ico was missing. Added the icon and reran CI (42351480621, pending at this update). No installer success claimed.

Added CSV patch import with explicit universe/address/footprint/kind, duplicate ID and overlap validation, and persistent passive venue mapping. All imported fixtures remain unverified, and frame generation is rejected even with an empty selection. UI replacement requires confirmation. Added jsdom component tests with mocked IPC (not desktop E2E).

Latest local validation: 18 Rust integration tests, 2 frontend component tests, strict TypeScript/Vite build pass. Next: inspect Windows build result, then backend profile approval registry, timed playback and real browser/desktop tests.

## TЗ 1.1: usable-chain work

Verified GitHub run 42351480621 succeeded and artifact exists (2,672,621 bytes). Download attempt returned HTTP 403 from artifact storage; installation/first-run testing has NOT occurred.

Added individual virtual fixture selection, backend monotonic timed Cue interpolation on a 25 ms worker, pause/stop, current-state Cue recording and SQLite sequence persistence, GO/manual Cue recall and startup project loading. Removed old three-fixture editor from primary UI to avoid competing controls. Twenty Rust tests pass. Physical output remains unavailable.

Blockers: no clean-Windows installation/IPC smoke evidence; scene positioning not integrated into playback; projects/Cues still single-slot storage; no named groups/sequence editing; no verified-profile registry; no Fixture Fingerprint yet. Next priority is Windows smoke workflow and desktop validation, then complete project-scoped Cue persistence before recognition expansion.
