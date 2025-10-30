fn main() {
    // Task: Print a short explanation of Rust.
    println!("Rust is a systems programming language focused on safety and performance.");

    // Challenge: Print 3 real-world projects using Rust.
    println!("Projects using Rust:");
    println!("1. Firefox");
    println!("2. Dropbox");
    println!("3. ripgrep");

    // Bonus Challenge: Call a function that returns a formatted description.
    println!("{}", describe_rust());
}

fn describe_rust() -> String {
    "Rust is fast, memory-safe, and prevents data races without a garbage collector.".to_string()
}