#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_personalized_greeting() {
        let greeting = personalized_greeting("Andrei");
        assert_eq!(greeting, "Welcome to Rust, Andrei! Let's build something amazing.");
        // Expected output: "Welcome to Rust, Andrei! Let's build something amazing."
    }

    #[test]
    fn test_argument_handling() {
        let args = vec!["program".to_string(), "Alice".to_string()];
        let name = &args[1];
        assert_eq!(name, "Alice");
        // Expected output: "Hello, Alice!"
    }
}