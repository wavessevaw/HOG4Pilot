# Test report — 2026-11-07

`cargo test --locked`: PASS, 13 integration tests (10 core, 2 CSV import, 1 SQLite persistence).
`npm run build`: PASS, strict TypeScript checking and Vite production bundle.

Core tests include loopback UDP Art-Net, invalid numeric values, conflicting addresses/sources, locked nonzero neutral values, fade and simulated connection loss.
SQLite test reopens a file-backed database and verifies that rejected invalid writes preserve the previous valid project.
CSV tests ensure unverified imports and rejection of unsupported/ambiguous channel descriptions.

Not tested: Tauri compilation/runtime, browser interaction, Windows installer, sACN, Hog, physical devices. Linux GTK/WebKit development libraries are absent (`pkg-config` check failed). No Git remote configured, therefore GitHub Actions has not run.

Venue iteration: 16 Rust integration tests pass. Added full 24-fixture mapping→grouping→frame→SQLite recovery test, all-unknown completion, hazer/unknown refusal. Frontend TypeScript/Vite build passes. GUI interaction and hardware mapping remain unverified.

Latest iteration: `cargo test --locked` passed 18 tests; `npm test` passed 2 jsdom component tests; `npm run build` passed. UI tests cover disabled identification for unknown devices, unknown answer IPC payload and visible backend errors. They mock IPC and do not establish desktop integration. Windows icon repair CI 42351480621 still building at this report update.

TЗ1.1 iteration: 20 Rust tests pass, including individual fade, unaffected fixture preservation, pause, stop, hazardous target refusal and Cue persistence. Existing frontend tests remain mocked IPC only. Windows run 42351480621 succeeded with nonempty installer; no clean installation or first-run evidence. Download attempt failed HTTP403.

UFR: 26 Rust tests pass, including six fingerprint/library cases: cross-manufacturer match, equal-size incompatible Reset mapping, missing neutral/ranges, malformed Fine/overlapping ranges, semantic/neutral mismatch, immutable version persistence and passive candidate evidence. Frontend recognition tests cover incompatible results and unknown bypass. No physical recognition tests.
