mod macros;
mod logger;
mod automation;
mod indexer;

use std::time::Instant;

fn main() {
    // Start the universal clock
    let start = Instant::now();

    // Run macro demonstrations
    macros::run_macro_demos();

    // End the universal clock and print elapsed time
    let duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);

    // Log execution time to a file (plain text)
    logger::log_exec_time_plain(duration.as_secs_f64());

    // Log execution time to workspace_index.json (JSON)
    logger::log_execution_time(duration.as_secs_f64());

    // Generate workspace index after execution
    indexer::update_workspace_index();

    // Optionally, auto-commit and push (uncomment to enable)
    // automation::auto_commit_and_push();
}