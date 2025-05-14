# Rust-Macros

This repository is a playground and training ground for experimenting with Rust macros, code instrumentation, and execution time measurement. It is designed for AI ingestion and further automation, with a focus on clear documentation and traceability of all code and dependencies.

---

## Project Structure

```
Macros/
│
├── macros.rs        # Main Rust source file, all logic and macros live here
├── exec_times.log   # Appended log of execution times for each run (auto-generated)
└── README.md        # This file, always up-to-date with project state
```

---

## Current Functionality

- **Macro Experiments:**  
  Demonstrates Rust macro_rules! for patterns, types, and trait-associated items.
- **Execution Time Measurement:**  
  Uses `std::time::Instant` to measure and print the total execution time of the program.
- **Execution Time Logging:**  
  Appends each run's execution time (in seconds) to `exec_times.log` for later analysis (e.g., histogram).
- **Minimal Dependencies:**  
  Only uses Rust standard library (`std::cell`, `std::time`, `std::fs`, `std::io`).

---

## File System State

- **`macros.rs`:**  
  - All code, macros, and logic are contained in this single file.
  - No external modules or crates are used.
  - All statements and function calls are inside `main()`.
- **`exec_times.log`:**  
  - Created automatically if it does not exist.
  - Each line is a floating-point number representing the execution time of a run.
- **`README.md`:**  
  - This file, kept up-to-date for AI and human readers.

---

## Function Calling & Invocation

- **Entry Point:**  
  - `fn main()` is the entry point for all logic.
- **Macro Usage:**  
  - Macros are defined and invoked within `main()` for demonstration.
- **Trait Example:**  
  - Shows how to use macros for trait-associated constants.

---

## Allocation & Logging

- **Memory Allocation:**  
  - Uses `Vec` and `RefCell` for demonstration.
- **Execution Time Logging:**  
  - Uses `OpenOptions` and `Write` from `std::fs` and `std::io` to append to `exec_times.log`.

---

## Dependencies

| Dependency         | Where Called         | Where Used         | Purpose                               |
|--------------------|---------------------|--------------------|---------------------------------------|
| `std::cell::RefCell` | Top of `macros.rs` | Global/thread local| Demonstrates thread-local storage     |
| `std::time::Instant` | Top of `macros.rs` | `main()`           | Execution time measurement            |
| `std::fs::OpenOptions` | Top of `macros.rs` | `main()`           | Open/create log file                  |
| `std::io::Write`      | Top of `macros.rs` | `main()`           | Write execution time to log           |

---

## GitHub Integration

- Repository: [Rust-Macros](https://github.com/Tecnocrat/Rust-Macros)
- To sync local changes:
  ```sh
  git add macros.rs exec_times.log README.md
  git commit -m "Update macros and logs"
  git push
  ```

---

## Updating This README

This file is the single source of truth for the project state.  
**Always update this README.md when:**
- Adding new macros or features
- Changing dependencies
- Modifying file structure
- Adding new logging or instrumentation

---

## Future Directions

- Add JSON or other structured logs for richer AI ingestion.
- Build a workspace index for codebase analysis.
- Expand macro demonstrations and performance instrumentation.
- Integrate with GitHub Actions for CI/CD and automated analysis.

---

*This README is designed for both AI and human consumption. Keeping it updated ensures seamless automation and onboarding for future contributors and AI agents.*
