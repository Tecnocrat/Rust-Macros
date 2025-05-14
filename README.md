# Rust-Macros

This repository is a playground and automation hub for Rust macro experimentation, code instrumentation, execution time measurement, and workspace introspection. It is designed for both AI and human consumption, with robust logging, GitHub automation, and semantic indexing.

---

## Project Structure

```
Macros/
│
├── src/
│   ├── main.rs          # Rust entry point, orchestrates macro demos and logging.
│   ├── macros.rs        # Macro implementations and usage examples.
│   ├── logger.rs        # Logging utilities (JSON, exec times, metadata).
│   ├── automation.rs    # GitHub automation (auto-commit, push).
│   ├── indexer.rs       # Workspace index and metadata generation.
│
├── python/
│   ├── main.py          # Python automation, semantic analysis, and orchestration.
│   ├── update_metadata.py # Updates workspace index and manifest.
│   ├── codebot_api.py   # Flask API for workspace index and manifest.
│
├── exec_times.log       # Log of execution times per run.
├── workspace_index.json # JSON metadata for VSCode indexing.
├── codebot.json         # Manifest for AI tools.
├── git_log_db.json      # Git commit/semantic history.
├── README.md            # Repository documentation.
├── Cargo.toml           # Rust project manifest.
├── .gitignore           # Ignore build, logs, and Python cache.
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

## Usage

- **Rust:**  
  `cargo run` (from project root)
- **Python:**  
  `python/python/main.py` or `python/python/update_metadata.py`
- **API:**  
  `python/python/codebot_api.py` (Flask, serves `/index` and `/manifest`)
- **CI/CD:**  
  Automated on push and daily via GitHub Actions.

---

## Contribution & Maintenance

- Keep this README up-to-date with all changes.
- Update `workspace_index.json` and `codebot.json` after significant changes.
- Use relative paths for all scripts and logs.
- Archive or rotate logs as needed.

---

## Future Directions

- Expand macro demonstrations and semantic analysis.
- Add log rotation/archival.
- Enhance cross-language introspection.
- Integrate more advanced CI/CD and AI workflows.

---

*This README is the single source of truth for the project. Keeping it updated ensures seamless onboarding and automation for all contributors and AI agents.*
