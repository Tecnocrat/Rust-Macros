use std::process::Command;

pub fn auto_commit_and_push() {
    // Set git user config before committing
    Command::new("git")
        .args(&["config", "--global", "user.name", "tecnocrat"])
        .status()
        .expect("Failed to set git user.name");

    Command::new("git")
        .args(&["config", "--global", "user.email", "jesussard@gmail.com"])
        .status()
        .expect("Failed to set git user.email");

    let timestamp = chrono::Local::now().to_string();
    let message = format!("Auto: log, index, and manifest update [{}]", timestamp);

    Command::new("git")
        .args(&["add", "."])
        .status()
        .expect("Failed to stage changes");

    Command::new("git")
        .args(&["commit", "-m", &message])
        .status()
        .expect("Failed to commit changes");

    Command::new("git")
        .args(&["push"])
        .status()
        .expect("Failed to push changes");

    println!("Auto-commit & push completed.");
}