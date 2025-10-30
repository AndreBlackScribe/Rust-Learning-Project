fn main() {
    // Task: Ownership transfer
    let numbers = vec![1, 2, 3];
    print_vector(numbers);

    // Uncommenting the next line would cause a compile error due to ownership transfer
    // println!("{:?}", numbers);

    // Challenge: Modify and return vector
    let mut more_numbers = vec![4, 5, 6];
    more_numbers = modify_vector(more_numbers);
    println!("Modified vector: {:?}", more_numbers);

    // Bonus Challenge: Clone before passing
    let original = vec![7, 8, 9];
    let cloned = original.clone();
    print_vector(cloned);
    println!("Original vector still accessible: {:?}", original);
}

fn print_vector(v: Vec<i32>) {
    println!("Vector inside function: {:?}", v);
}

fn modify_vector(mut v: Vec<i32>) -> Vec<i32> {
    v.push(100);
    v
}