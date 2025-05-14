use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;
use serde_json::json;
use crate::logger::get_git_commit_hash;

pub fn update_workspace_index() {
    let paths = fs::read_dir(".").unwrap();
    let mut files = Vec::new();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            files.push(json!({ "name": path.to_string_lossy() }));
        }
    }
    let metadata = json!({
        "project_name": "Rust-Macros",
        "files": files,
        "last_commit": get_git_commit_hash(),
        // Optionally, add more metadata here
    });

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("workspace_index.json")
        .expect("Failed to open index file");

    writeln!(file, "{}", metadata).expect("Failed to write workspace index");
}

pub fn update_workspace_index_with_python() {
    let status = Command::new("python")
        .args(&["python/update_metadata.py"])
        .status()
        .expect("Failed to run Python indexer");
    if !status.success() {
        eprintln!("Python indexer failed");
    }
}