# Изменения

## 0.1.0 — 2026-11-07, начальное ядро

Добавлены профиль виртуального RGB Dimmer, Safety Engine, генерация полного кадра, интерполяция, симулятор и UDP Art-Net адаптер. Добавлены автоматические негативные проверки и loopback тест. Реальные приборы и Road Hog 4 не проверены. Этап Core выполнен частично: постоянное хранение и desktop оболочка остаются в работе.

## Development continuation — 2026-11-07

Added SQLite transactional project storage, prior revision retention, saved-look validation, React editor and Tauri IPC sources, Windows NSIS workflow, conservative CSV profile import and three additional integration tests. Frontend build and 13 Rust tests pass. Tauri/Windows compilation not yet verified.

## Venue Auto-Mapping prototype

Added persistent wizard for a 24-fixture synthetic patch, unknown answers, safe per-fixture simulation, coverage grouping and warm-look simulation. Unknown fixtures and hazer remain passive. Added three integration tests.

## Patch import and UI tests

Fixed missing Windows icon resource. Added passive CSV patch import with address conflict detection, persisted variable-size venue mapping, and explicit replacement confirmation. Added UI component tests to Windows CI. Imported data cannot grant output permission.

## Individual virtual fixtures and timed Cue playback

Primary editor now selects individual synthetic fixtures, records snapshots to SQLite and recalls ordered Cues with Rust monotonic interpolation. Added backend worker, pause, stop and startup project load. Hardware output remains disabled.
