#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_day_name() {
        let day_number = 3;
        let day_name = match day_number {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            7 => "Sunday",
            _ => "Invalid day",
        };
        assert_eq!(day_name, "Wednesday");
        // Expected output: "Day 3 is Wednesday"
    }

    #[test]
    fn test_invalid_day_name() {
        let day_number = 9;
        let day_name = match day_number {
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            7 => "Sunday",
            _ => "Invalid day",
        };
        assert_eq!(day_name, "Invalid day");
        // Expected output: "Day 9 is Invalid day"
    }

    #[test]
    fn test_day_message() {
        assert_eq!(day_message(1), "Start of the week!");
        assert_eq!(day_message(4), "Workday");
        assert_eq!(day_message(6), "Weekend!");
        assert_eq!(day_message(9), "Unknown day");
        // Expected output:
        // day_message(1) => "Start of the week!"
        // day_message(4) => "Workday"
        // day_message(6) => "Weekend!"
        // day_message(9) => "Unknown day"
    }
}