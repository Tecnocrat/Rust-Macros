# Rust-Macros

**Cross-language automation, semantic introspection, and AI-ready metadata for Rust/Python codebases.**  
This repository is a playground and automation hub for Rust macro experimentation, code instrumentation, execution time measurement, and workspace introspection. It is designed for both AI and human consumption, with robust logging, GitHub automation, and semantic indexing.

---

## Project Structure

```
Macros/
│
├── src/
│   ├── main.rs          # Rust orchestrator: macros, logging, Python integration, automation.
│   ├── macros.rs        # Macro demos and utilities.
│   ├── logger.rs        # Logging (plain, JSON, commit hash).
│   ├── automation.rs    # Git auto-commit/push.
│   ├── indexer.rs       # Calls Python for advanced indexing.
│
├── python/
│   ├── main.py                  # Python: semantic analysis, workspace index, git log, helpers.
│   ├── update_metadata.py       # Python: runs index+manifest generation.
│   ├── codebot_api.py           # Python: Flask API for index/manifest.
│
├── exec_times.log       # Plain execution time log.
├── workspace_index.json # JSON index (Python or Rust generated).
├── codebot.json         # AI manifest (Python generated).
├── git_log_db.json      # Git commit/semantic history (Python generated).
├── README.md            # Dense, up-to-date documentation.
├── Cargo.toml           # Rust manifest.
├── .gitignore           # Ignore build, logs, Python cache.
│
└── .github/
    └── workflows/
        ├── ci.yml       # Rust CI/CD & auto-logging.
        └── metadata.yml # Python index/manifest automation.
```

---

## Key Features

- **Macro Experiments:**  
  Rust macro_rules! and trait-associated items, with demos in `macros.rs`.
- **Execution Time Measurement:**  
  Uses `std::time::Instant` in Rust and logs to `exec_times.log`.
- **Structured Logging:**  
  JSON logs and workspace metadata for AI and CI/CD.
- **Automated GitHub Maintenance:**  
  Auto-commit and push after each run (Rust and Python).
- **Workspace Indexing:**  
  Semantic summaries and file metadata in `workspace_index.json`.
- **API Access:**  
  Flask API (`python/codebot_api.py`) serves index and manifest for external tools.
- **CI/CD:**  
  - `.github/workflows/ci.yml`: Runs Rust code, logs, and pushes updates.
  - `.github/workflows/metadata.yml`: Runs Python scripts for index/manifest.

---

## Runtime Flow

1. **Rust (`main.rs`):**
   - Runs macro demos, logs execution time (plain + JSON).
   - Calls Python (`python/update_metadata.py`) for:
     - Recursive workspace index with semantic summaries.
     - Manifest generation for AI tools.
   - Optionally triggers auto-commit/push.
2. **Python:**
   - `main.py`: Semantic analysis, recursive index, git log, helpers.
   - `update_metadata.py`: Runs index + manifest generation.
   - `codebot_api.py`: Flask API for index/manifest.
3. **Artifacts:**
   - `workspace_index.json`: All files, sizes, timestamps, semantic summaries.
   - `codebot.json`: Project, platform, language, file summaries, AI metadata.
   - `git_log_db.json`: Commit/semantic history.

---

## Key Functions

- **Rust**
  - `macros::run_macro_demos()`: Macro playground.
  - `logger::log_exec_time_plain(f64)`: Execution time log.
  - `logger::log_execution_time(f64)`: Execution time + commit hash (JSON).
  - `indexer::update_workspace_index_with_python()`: Calls Python for deep index/manifest.
  - `automation::auto_commit_and_push()`: Git add/commit/push.
- **Python**
  - `generate_recursive_workspace_index_with_summaries()`: Recursively indexes all files, adds semantic summaries for Python files.
  - `generate_codebot_manifest()`: Generates dense AI manifest.
  - `save_git_log_database(repo_path, file_path)`: Updates git/semantic history.
  - `auto_commit_and_push()`: Git automation.
  - `codebot_api.py`: Flask API for index/manifest.

---

## Context Tracking & Metadata

- All logs and indexes include commit hash, timestamp, and semantic summaries.
- Manifest and index are always up-to-date after each run.
- Python and Rust workflows are fully integrated; Rust can trigger all Python metadata updates.
- All metadata is AI/LLM-ready for downstream automation.

---

## Quickstart

1. **Run full workflow:**  
   `cargo run`  
   (Triggers Rust macros, logging, Python indexing, and manifest generation.)

2. **Update metadata only:**  
   `python python/update_metadata.py`

3. **Serve API:**  
   `python python/codebot_api.py`

4. **CI/CD:**  
   Automated on push and schedule via GitHub Actions.

---

## Contribution & Maintenance

- Keep this README up-to-date with all changes.
- Update `workspace_index.json` and `codebot.json` after significant changes.
- Use relative paths for all scripts and logs.
- Archive or rotate logs as needed.

---

## Extensibility

- Add new semantic analyzers in Python for other languages.
- Extend Rust orchestrator to trigger more Python or external tools.
- All metadata is AI/LLM-ready for downstream automation.

---

## Future Directions

- Expand macro demonstrations and semantic analysis.
- Add log rotation/archival.
- Enhance cross-language introspection.
- Integrate more advanced CI/CD and AI workflows.

---

*This README is the canonical, dense reference for all contributors and AI agents. All context, metadata, and automation flows are described herein. Keeping it updated ensures seamless onboarding and automation for all contributors and AI agents.*
