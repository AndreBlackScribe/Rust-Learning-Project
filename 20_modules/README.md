## Topic: Modules

### Task
Create a module named `greetings` with a public function `hello()` that prints a message.

### Challenge
Create a nested module `math::operations` with a function that adds two numbers.

### Hint
Use `mod greetings;` in `main.rs` and define `pub fn hello()` in `greetings.rs`. For nested modules, use `pub mod operations;` inside `math.rs`.

### Bonus Challenge
Use `use` statements to simplify access to deeply nested functions.