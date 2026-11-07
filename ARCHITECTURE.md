# HOG4Pilot architecture

Rust core remains the authority for validating frames. React sends normalized intents through Tauri commands. No network adapter is exposed by IPC. Desktop currently renders a synthetic three-fixture universe and displays the frame returned by SafetyEngine.

`project::Store` owns SQLite, enables WAL, saves the current project and previous revision in one transaction. `Project::validate` runs on both write and read. Schema version: 1. `import::dmx_csv` accepts explicit continuous channels and rejects unsupported functions; imported profiles remain unverified.

Tauri owns a mutex-protected store and engine, opens the database under app_data_dir, and exposes save/load/preview/stop. Stop latches until process restart. This is not a physical emergency stop.

See docs/architecture.md for core contracts. Still required: independent playback/output process and watchdog, structured verification provenance, migrations beyond v1, venue/profile persistence, output ownership policy, calibration, discovery, Hog adapter.
