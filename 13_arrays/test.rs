#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_sum() {
        let numbers = [3, 7, 2, 9, 4];
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 25);
        // Expected output: "Sum: 25"
    }

    #[test]
    fn test_max_min_values() {
        let numbers = [3, 7, 2, 9, 4];
        let max = numbers.iter().max().unwrap();
        let min = numbers.iter().min().unwrap();
        assert_eq!(*max, 9);
        assert_eq!(*min, 2);
        // Expected output:
        // Max: 9
        // Min: 2
    }

    #[test]
    fn test_average_function() {
        let numbers = [3, 7, 2, 9, 4];
        let avg = average(&numbers);
        assert!((avg - 5.0).abs() < 0.01);
        // Expected output: "Average: 5.00"
    }
}