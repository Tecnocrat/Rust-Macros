use std::cell::RefCell;
use std::time::Instant;
use std::fs::{self, File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::Path;
thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

// Associated item in a trait
macro_rules! const_maker {
    ($t:ty, $v:tt) => { const CONST: $t = $v; };
}
trait T {
    const_maker! {i32, 7}
}

fn main() {
    // Start the universal clock
    let start = Instant::now();

    // Expression
    let numbers = vec![1, 2, 3];

    // Statement
    println!("Hello!");

    // Pattern
    macro_rules! pat {
        ($i:ident) => (Some($i))
    }
    if let pat!(x) = Some(42) {
        println!("{}", x);
    }

    // Type
    macro_rules! Tuple {
        { $A:ty, $B:ty } => { ($A, $B) };
    }
    type Pair = Tuple!(i32, f64);

    // Simulate some work
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    // End the universal clock and print elapsed time
    let duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);

    // Log execution time to a file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("exec_times.log")
        .expect("Unable to open log file");
    writeln!(file, "{:.6}", duration.as_secs_f64()).expect("Unable to write to log file");

    // Generate workspace index after execution
    generate_workspace_index();
}

fn generate_workspace_index() {
    let paths = fs::read_dir(".").unwrap();
    let mut index = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("workspace_index.json")
            .unwrap(),
    );
    writeln!(index, "{{\n  \"files\": [").unwrap();
    let mut first = true;
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            if !first { writeln!(index, ",").unwrap(); }
            write!(index, "    {{ \"name\": \"{}\" }}", path.display()).unwrap();
            first = false;
        }
    }
    writeln!(index, "\n  ]\n}}").unwrap();
}
