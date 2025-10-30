#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_modification() {
        let mut fruits = vec!["apple".to_string(), "banana".to_string()];
        fruits.push("orange".to_string());
        fruits.push("banana".to_string());
        fruits.remove(0);
        assert_eq!(fruits, vec!["banana", "orange", "banana"]);
        // Expected output: Modified vector: ["banana", "orange", "banana"]
    }

    #[test]
    fn test_sort_and_dedup() {
        let mut fruits = vec!["banana".to_string(), "orange".to_string(), "banana".to_string()];
        fruits.sort();
        fruits.dedup();
        assert_eq!(fruits, vec!["banana", "orange"]);
        // Expected output: Sorted & deduplicated: ["banana", "orange"]
    }

    #[test]
    fn test_join_vector() {
        let fruits = vec!["banana".to_string(), "orange".to_string()];
        let result = join_vector(&fruits);
        assert_eq!(result, "banana, orange");
        // Expected output: Joined string: banana, orange
    }
}