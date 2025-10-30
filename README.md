# 🦀 Rust Learning Project

Welcome to the **Rust Learning Project** — a hands-on, challenge-driven journey through the fundamentals of the Rust programming language. This repository is designed to help you build confidence and mastery in Rust through practical tasks, coding challenges, and guided hints.

---

## 📚 Project Structure

Each topic is organized into its own folder:

```
rust_learning/
├── 01_introduction/
│   ├── README.md
│   └── main.rs
│   └── test.rs
├── 02_get_started/
│   ├── README.md
│   └── main.rs
│   └── test.rs
...
├── 20_modules/
│   ├── README.md
│   ├── main.rs
│   └── test.rs
```

Each folder contains:
- `README.md` — task, challenge, hint, and bonus challenge
- `main.rs` — your implementation
- `test.rs` — unit tests

---

## 🧠 Topics Covered

1. Introduction  
2. Get Started  
3. Syntax  
4. Comments  
5. Variables  
6. Data Types  
7. Operators  
8. Strings  
9. Booleans  
10. If...Else  
11. Loops  
12. Match  
13. Arrays  
14. Vectors  
15. Functions  
16. Ownership  
17. References  
18. Structs  
19. Enums  
20. Modules  

---

## 🚀 How to Use
0. It is assumed that you allready have installed rust on your linux machine. if not follow: https://rust-lang.org/tools/install/
1. **Clone the repository**  
   ```bash
   git clone https://github.com/AndreBlackScribe/Rust-Learning-Project.git
   cd Rust-Learning-Project
   ```

2. **Navigate to a topic folder**  
   ```bash
   cd 15_functions
   ```
2b. **Initialize a Cargo project (if needed)**
If you want to run or test the code using Cargo, initialize the folder as a Cargo project.
⚠️ Since folder names start with digits, use the --name flag to specify a valid package name:
   ```bash
   cargo init --bin --name functions_15
   mv main.rs src/main.rs
   mv test.rs src/test.rs  # or integrate into main.rs
   ```

3. **Run the code**  
   ```bash
   cargo run
   ```

4. **Run tests** (for topics 15–20)  
   ```bash
   cargo test
   ```

---

## 🛠 Requirements

- Rust
- Cargo (comes with Rust)

---

## ✨ Contributing

Feel free to fork this project, add your own challenges, or improve the existing ones. Pull requests are welcome!

---

## 📖 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## 🙌 Acknowledgments

Created and maintained by Andrei Filipiuc as a personal learning journey and open resource for the Rust community.

---

Happy coding! 🦀
