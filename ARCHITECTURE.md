# HOG4Pilot architecture

Rust core remains the authority for validating frames. React sends normalized intents through Tauri commands. No network adapter is exposed by IPC. Desktop currently renders a synthetic three-fixture universe and displays the frame returned by SafetyEngine.

`project::Store` owns SQLite, enables WAL, saves the current project and previous revision in one transaction. `Project::validate` runs on both write and read. Schema version: 1. `import::dmx_csv` accepts explicit continuous channels and rejects unsupported functions; imported profiles remain unverified.

Tauri owns a mutex-protected store and engine, opens the database under app_data_dir, and exposes save/load/preview/stop. Stop latches until process restart. This is not a physical emergency stop.

See docs/architecture.md for core contracts. Still required: independent playback/output process and watchdog, structured verification provenance, migrations beyond v1, venue/profile persistence, output ownership policy, calibration, discovery, Hog adapter.

Patch import contract: `Venue::import_patch` accepts UTF-8 CSV header `id,universe,address,channels,kind`; fields cannot contain commas. Universe uses Art-Net numbering 0–32767; address is 1-based. Kind is light/hazer/unknown. Imported venues have simulation=false, all fixtures unverified. `validate_passive` enforces IDs, footprints and conflicts within each universe. `frame` refuses all imported venues. `venue_import` validates before saving and returns the persisted model. This is patch ingestion, not RDM discovery or profile verification.
