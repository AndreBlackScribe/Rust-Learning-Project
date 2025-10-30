fn main() {
    let a = 12;
    let b = 4;

    // Task: Basic arithmetic operations
    println!("Addition: {}", a + b);
    println!("Subtraction: {}", a - b);
    println!("Multiplication: {}", a * b);
    println!("Division: {}", a / b);

    // Challenge: Modulus and exponentiation
    println!("Modulus: {}", a % b);
    println!("Exponentiation: {}", i32::pow(a, b));

    // Bonus Challenge: Operator-based function
    println!("Using operator '+': {}", calculate(a, b, "+"));
    println!("Using operator '*': {}", calculate(a, b, "*"));
}

fn calculate(x: i32, y: i32, op: &str) -> i32 {
    match op {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        "%" => x % y,
        "^" => x.pow(y as u32),
        _ => {
            println!("Unknown operator");
            0
        }
    }
}