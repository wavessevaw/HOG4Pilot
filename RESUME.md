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
