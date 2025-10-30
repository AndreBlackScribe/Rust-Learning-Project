#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_to_n() {
        assert_eq!(sum_to_n(10), 55);
        assert_eq!(sum_to_n(5), 15);
        // Expected output:
        // Sum from 1 to 10: 55
        // Sum from 1 to 5: 15
    }

    #[test]
    fn test_loop_counts() {
        let mut count = 0;
        for _ in 1..=10 {
            count += 1;
        }
        assert_eq!(count, 10);
        // Expected output: 10 iterations
    }
}