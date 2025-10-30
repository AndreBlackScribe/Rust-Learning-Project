fn main() {
    // Task: Declare and print array elements
    let numbers = [3, 7, 2, 9, 4];
    println!("Array elements:");
    for num in numbers.iter() {
        println!("{}", num);
    }

    // Calculate sum
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    // Challenge: Find max and min
    let max = numbers.iter().max().unwrap();
    let min = numbers.iter().min().unwrap();
    println!("Max: {}", max);
    println!("Min: {}", min);

    // Bonus Challenge: Calculate average
    let avg = average(&numbers);
    println!("Average: {:.2}", avg);
}

fn average(arr: &[i32]) -> f64 {
    let sum: i32 = arr.iter().sum();
    sum as f64 / arr.len() as f64
}