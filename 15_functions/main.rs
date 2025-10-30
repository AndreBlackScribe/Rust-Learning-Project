fn main() {
    // Task: Sum function
    let a = 5;
    let b = 3;
    let result = sum(a, b);
    println!("Sum of {} and {} is {}", a, b, result);

    // Challenge: Factorial
    let n = 5;
    println!("Factorial of {} is {}", n, factorial(n));

    // Bonus Challenge: Sum and product as tuple
    let (s, p) = sum_and_product(a, b);
    println!("Sum: {}, Product: {}", s, p);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn sum_and_product(x: i32, y: i32) -> (i32, i32) {
    (x + y, x * y)
}