use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;

pub fn log_execution_time(exec_time: f64) {
    let log_entry = json!({
        "timestamp": chrono::Local::now().to_string(),
        "execution_time": exec_time,
        "commit_hash": get_git_commit_hash()
    });

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("workspace_index.json")
        .expect("Failed to open log file");

    writeln!(file, "{}", log_entry).expect("Failed to write execution log");
}