# Test report — 2026-11-07

`cargo test --locked`: PASS, 13 integration tests (10 core, 2 CSV import, 1 SQLite persistence).
`npm run build`: PASS, strict TypeScript checking and Vite production bundle.

Core tests include loopback UDP Art-Net, invalid numeric values, conflicting addresses/sources, locked nonzero neutral values, fade and simulated connection loss.
SQLite test reopens a file-backed database and verifies that rejected invalid writes preserve the previous valid project.
CSV tests ensure unverified imports and rejection of unsupported/ambiguous channel descriptions.

Not tested: Tauri compilation/runtime, browser interaction, Windows installer, sACN, Hog, physical devices. Linux GTK/WebKit development libraries are absent (`pkg-config` check failed). No Git remote configured, therefore GitHub Actions has not run.

Venue iteration: 16 Rust integration tests pass. Added full 24-fixture mapping→grouping→frame→SQLite recovery test, all-unknown completion, hazer/unknown refusal. Frontend TypeScript/Vite build passes. GUI interaction and hardware mapping remain unverified.
