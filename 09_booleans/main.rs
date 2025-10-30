fn main() {
    let number = 7;

    // Task: Check if the number is even or odd
    let is_even = number % 2 == 0;
    println!("Is {} even? {}", number, is_even);

    // Challenge: Check if divisible by both 3 and 5
    let test_number = 15;
    println!(
        "Is {} divisible by both 3 and 5? {}",
        test_number,
        divisible_by_3_and_5(test_number)
    );

    // Bonus Challenge: Boolean to message
    println!("Boolean message: {}", boolean_message(true));
    println!("Boolean message: {}", boolean_message(false));
}

fn divisible_by_3_and_5(n: i32) -> bool {
    n % 3 == 0 && n % 5 == 0
}

fn boolean_message(flag: bool) -> &'static str {
    if flag {
        "Yes"
    } else {
        "No"
    }
}