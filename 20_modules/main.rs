mod greetings;
mod math;

use math::operations::add;

fn main() {
    // Task: Call function from greetings module
    greetings::hello();

    // Challenge: Call nested module function
    let result = add(2, 3);
    println!("2 + 3 = {}", result);

    // Bonus Challenge: Using `use` to simplify access
    use greetings::hello as greet;
    greet();
}