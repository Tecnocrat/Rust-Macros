use std::fs::{OpenOptions, File};
use std::io::{Write, BufWriter};
use serde_json::json;

pub fn log_exec_time_plain(exec_time: f64) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("exec_times.log")
        .expect("Unable to open log file");
    writeln!(file, "{:.6}", exec_time).expect("Unable to write to log file");
}

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

// Dummy implementation for get_git_commit_hash
pub fn get_git_commit_hash() -> String {
    // In production, use `git rev-parse HEAD` or similar
    "dummy_commit_hash".to_string()
}