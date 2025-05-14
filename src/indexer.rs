use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;

pub fn update_workspace_index() {
    let metadata = json!({
        "project_name": "Rust-Macros",
        "files_tracked": ["macros.rs", "logger.rs", "automation.rs"],
        "last_commit": get_git_commit_hash(),
        "execution_history": load_exec_times()
    });

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("workspace_index.json")
        .expect("Failed to open index file");

    writeln!(file, "{}", metadata).expect("Failed to write workspace index");
}