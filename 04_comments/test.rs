#[cfg(test)]
mod tests {
    #[test]
    fn test_comment_example() {
        let greeting = "Hello from Rust!";
        assert_eq!(greeting, "Hello from Rust!");
        // Expected output: "Hello from Rust!"
    }

    #[test]
    fn test_memory_safety_guide_keywords() {
        let guide = "Ownership, Borrowing, Lifetimes";
        assert!(guide.contains("Ownership"));
        assert!(guide.contains("Borrowing"));
        assert!(guide.contains("Lifetimes"));
        // Expected output: Guide includes "Ownership", "Borrowing", and "Lifetimes"
    }
}