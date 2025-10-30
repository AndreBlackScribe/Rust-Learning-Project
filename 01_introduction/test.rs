#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_rust() {
        let description = describe_rust();
        assert!(description.contains("memory-safe"));
        // Expected output: "Rust is fast, memory-safe, and prevents data races without a garbage collector."
    }

    #[test]
    fn test_project_names() {
        let projects = vec!["Firefox", "Dropbox", "ripgrep"];
        assert_eq!(projects.len(), 3);
        assert!(projects.contains(&"Firefox"));
        // Expected output:
        // 1. Firefox
        // 2. Dropbox
        // 3. ripgrep
    }
}