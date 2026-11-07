# Resume

Work in /workspace/HOG4Pilot. Read DEVELOPMENT_STATUS.md, TEST_REPORT.md and ROADMAP.md. Do not replace the core or claim v1.0 readiness.

Toolchain in this environment:
```
export CARGO_HOME=/workspace/.cargo
export RUSTUP_HOME=/workspace/.rustup
export PATH=/workspace/.cargo/bin:$PATH
cargo test --locked
npm ci --cache /workspace/.npm-cache
npm run build
npm run tauri dev
```

Desktop Linux compilation needs GTK3/WebKit2GTK 4.1 development packages. Windows workflow is .github/workflows/windows.yml; no remote is configured and it has not run. Obtain actual successful runner evidence before claiming Windows readiness. React browser mode intentionally reports IPC errors instead of faking a Rust backend.

Continue with desktop compile and backend playback worker. Maintain tests and commit each completed block. Active physical control requires separate user authorization and verified configuration.

Latest: remote wavessevaw/HOG4Pilot found. Venue wizard implemented for synthetic patch only. Next: inspect Windows CI result, then add imported patch input with immutable profile verification, timed identification expiry, real UI tests and individual-zone playback. RDM discovery and moving-head calibration remain absent.

Remote is now configured and code is published on main. First Windows run: 42347148551. Check it before any installer claim (`gh run view 42347148551 --repo wavessevaw/HOG4Pilot`). Local branch master was pushed to remote main; use `git push origin HEAD:main`. Do not force-push.

Latest continuation: icon resource fixed; inspect CI 42351480621 (icon commit 1d2df64) and newer runs. Passive patch importer and frontend component tests implemented. Run `cargo test --locked`, `npm test`, `npm run build`. No imported profile can output. Next implement profile approval registry and timed output worker after resolving actual Windows CI failures.

TЗ1.1 latest: src/playback.rs implements individual synthetic states and timed transitions; ShowControl.tsx is primary editor/playback. Check newest Windows CI after push. First confirmed successful installer is run 42351480621; artifact download here returned HTTP403, not tested on Windows. Next add installer launch/IPC smoke to Windows runner and validate project/Cue reload. Do not start Fingerprint before this basic chain works. No background agent running.

scripts/windows-smoke.ps1 now runs after Tauri build in Windows CI. Check its actual result; installation success must not be assumed. It covers process/window only, not IPC actions. TASK_QUEUE.md contains remaining P0 order.
