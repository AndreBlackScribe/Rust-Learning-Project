use std::env;

fn main() {
    // Task: Print "Hello, world!".
    println!("Hello, world!");

    // Challenge: Read a name from the command line and greet the user.
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let name = &args[1];
        println!("Hello, {}!", name);

        // Bonus Challenge: Use a function to generate a personalized greeting.
        println!("{}", personalized_greeting(name));
    } else {
        println!("No name provided. Usage: cargo run <name>");
    }
}

fn personalized_greeting(name: &str) -> String {
    format!("Welcome to Rust, {}! Let's build something amazing.", name)
}