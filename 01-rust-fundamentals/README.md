# 🦀 Rust Fundamentals

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Build-Cargo-informational?style=for-the-badge&logo=rust" alt="Cargo" />
  <img src="https://img.shields.io/badge/Course-Duke_University_%2F_Coursera-blue?style=for-the-badge" alt="Course" />
  <img src="https://img.shields.io/badge/Status-Completed-success?style=for-the-badge" alt="Status" />
</p>

A curated collection of hands-on labs, coding exercises, and practical implementations covering **Rust Fundamentals** (Rust Bootcamp / Duke University Specialization).

---

## 📌 Topics & Exercises Overview

| # | Topic | Concepts Covered | Source Code |
|:---:|:---|:---|:---:|
| **01** | **Components & Setup** | Project structure, `Cargo.toml`, manifest, bin/lib layout | [📂 View Code](./examples/1-components/) |
| **02** | **Variables & Mutability** | Immutable defaults, `let mut`, basic data types | [📂 View Code](./examples/2-variables/) |
| **03** | **Control Flow** | Conditional branches (`if`, `else`, `else if`) | [📂 View Code](./examples/3-control-flow/) |
| **04** | **Variable Shadowing** | Variable re-declaration, scope changes, type conversions | [📂 View Code](./examples/4-shadowing/) |
| **05** | **Basic Loops** | Unconditional loops, loop expressions | [📂 View Code](./examples/5-loops/) |
| **06** | **Conditionals** | Complex Boolean logic and expressions | [📂 View Code](./examples/6-conditionals/) |
| **07** | **While Loops** | Condition-driven loops, console user input | [📂 View Code](./examples/7-while-loops/) |
| **08** | **For Loops** | Iterating through collections, ranges (`0..n`) | [📂 View Code](./examples/8-for-loops/) |
| **09** | **Break & Continue** | Loop interruption, skipping iterations | [📂 View Code](./examples/9-break-and-continue/) |
| **10** | **Pattern Matching** | `match` expressions, exhaustive pattern matching | [📂 View Code](./examples/10-match-control-flow/) |
| **11** | **Unit Functions** | Defining functions, parameters, side effects | [📂 View Code](./examples/11-unit-functions/) |
| **12** | **Return Values** | Expression vs statement returns, implicit returns | [📂 View Code](./examples/12-return-values/) |
| **13** | **Function Arguments** | Passing arguments, ownership transfer by value | [📂 View Code](./examples/13-function-arguments/) |
| **14** | **Borrowing & References** | References (`&T`, `&mut T`), borrow checker rules | [📂 View Code](./examples/14-borrowing/) |
| **15** | **Panic Handling** | Unrecoverable errors, `panic!` macro | [📂 View Code](./examples/15-panic/) |
| **16** | **Error Handling** | Recoverable errors, `Result<T, E>`, `Option<T>` | [📂 View Code](./examples/16-error-handling/) |
| **17** | **Data Structures** | Custom structs, data aggregation | [📂 View Code](./examples/17-data-structures/) |
| 🧪 | **Practice Project Lab** | CLI File Reader Application with arguments & error handling | [📄 Lab Details](./lab.md) |

---

## 🚀 Getting Started

### Prerequisites
Make sure you have the official Rust toolchain installed:
```bash
# On Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows: install via https://rustup.rs
```

Verify your installation:
```bash
cargo --version
rustc --version
```

### Running Any Exercise
Navigate to any desired exercise directory and use Cargo to compile and run:

```bash
# Example: Running the Control Flow conditionals exercise
cd examples/3-control-flow/conditionals

# Check syntax without building
cargo check

# Compile and run
cargo run
```

---

## 🙏 Credits & Acknowledgments
The course structure, original starter templates, and curriculum are part of the **Rust Fundamentals / Rust Bootcamp** offered by **Duke University** on Coursera:
* **Course Instructor:** [Alfredo Deza](https://github.com/alfredodeza)
* **Reference Coursework:** [alfredodeza/rust-fundamentals](https://github.com/alfredodeza/rust-fundamentals)

---

## 👤 Author
- **H7 💜** ([@hashan-7](https://github.com/hashan-7))
