fn main() {
    // Task: Define and use enum
    let light = TrafficLight::Green;
    println!("Current light: {:?}", light);

    // Challenge: Call duration method
    println!("Duration: {} seconds", light.duration());

    // Bonus Challenge: Print message based on variant
    print_light_message(light);
}

#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn duration(&self) -> u32 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
}

fn print_light_message(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready..."),
        TrafficLight::Green => println!("Go!"),
    }
}