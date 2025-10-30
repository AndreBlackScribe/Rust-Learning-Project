fn main() {
    // This is a single-line comment.
    // The following line prints a greeting to the console.
    println!("Hello from Rust!");

    /*
        This is a multi-line comment.
        Rust is a systems programming language that emphasizes safety and performance.
        It achieves memory safety without needing a garbage collector.
    */

    // Challenge: Mini-guide on memory safety
    /*
        Rust ensures memory safety through:
        - Ownership: Each value has a single owner.
        - Borrowing: References can be passed without transferring ownership.
        - Lifetimes: Ensure references are valid for the duration they're used.
    */

    // Bonus Challenge: Ownership violation example
    /*
        let s1 = String::from("hello");
        let s2 = s1; // s1 is moved to s2, s1 is no longer valid
        println!("{}", s1); // This would cause a compile-time error
    */
}