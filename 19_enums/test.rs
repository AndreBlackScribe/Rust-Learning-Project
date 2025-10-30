#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        assert_eq!(TrafficLight::Red.duration(), 60);
        assert_eq!(TrafficLight::Yellow.duration(), 5);
        assert_eq!(TrafficLight::Green.duration(), 45);
    }

    #[test]
    fn test_print_light_message() {
        // These just print messages; no assertions needed
        print_light_message(TrafficLight::Red);
        print_light_message(TrafficLight::Yellow);
        print_light_message(TrafficLight::Green);
    }
}