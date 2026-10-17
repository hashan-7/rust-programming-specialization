# 🦀 Rust Programming Specialization

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Build-Cargo-informational?style=for-the-badge&logo=rust" alt="Cargo" />
  <img src="https://img.shields.io/badge/Course-Duke_University_%2F_Coursera-blue?style=for-the-badge" alt="Course" />
  <img src="https://img.shields.io/badge/Status-Completed-success?style=for-the-badge" alt="Status" />
</p>

A comprehensive repository containing hands-on labs, coding exercises, and real-world practical implementations from the **Rust Bootcamp Specialization** offered by **Duke University** on Coursera.

---

## 📚 Modules Overview

```text
RustCourse/
├── 📁 01-rust-fundamentals/         # Week 2: Core syntax, Control flow, Functions, Borrowing
├── 📁 02-rust-structs-types-enums/   # Week 3: Structs, Methods, Vectors, Strings, Enums
└── 📁 03-applied-rust/               # Week 4: Applied Rust, Library Development, CLI Tools & Testing
```

---

## 📦 Module 01: Rust Fundamentals

| # | Topic | Concepts Covered | Source Code |
|:---:|:---|:---|:---:|
| **01** | **Components & Setup** | Project structure, `Cargo.toml`, bin/lib layout | [📂 View Code](./01-rust-fundamentals/examples/1-components/) |
| **02** | **Variables & Mutability** | Immutability by default, `let mut`, basic types | [📂 View Code](./01-rust-fundamentals/examples/2-variables/) |
| **03** | **Control Flow** | Branching logic (`if`, `else`, `else if`) | [📂 View Code](./01-rust-fundamentals/examples/3-control-flow/) |
| **04** | **Variable Shadowing** | Variable re-declaration, scope changes | [📂 View Code](./01-rust-fundamentals/examples/4-shadowing/) |
| **05** | **Basic Loops** | Unconditional `loop` expressions | [📂 View Code](./01-rust-fundamentals/examples/5-loops/) |
| **06** | **Conditionals** | Complex Boolean logic and expressions | [📂 View Code](./01-rust-fundamentals/examples/6-conditionals/) |
| **07** | **While Loops** | Condition-driven loops, console input | [📂 View Code](./01-rust-fundamentals/examples/7-while-loops/) |
| **08** | **For Loops** | Iterating through collections and ranges (`0..n`) | [📂 View Code](./01-rust-fundamentals/examples/8-for-loops/) |
| **09** | **Break & Continue** | Loop interruption and skipping iterations | [📂 View Code](./01-rust-fundamentals/examples/9-break-and-continue/) |
| **10** | **Pattern Matching** | Exhaustive `match` control flow expressions | [📂 View Code](./01-rust-fundamentals/examples/10-match-control-flow/) |
| **11** | **Unit Functions** | Defining functions, parameters, side effects | [📂 View Code](./01-rust-fundamentals/examples/11-unit-functions/) |
| **12** | **Return Values** | Expression vs statement returns, implicit returns | [📂 View Code](./01-rust-fundamentals/examples/12-return-values/) |
| **13** | **Function Arguments** | Passing arguments, ownership transfer by value | [📂 View Code](./01-rust-fundamentals/examples/13-function-arguments/) |
| **14** | **Borrowing & References** | References (`&T`, `&mut T`), borrow checker rules | [📂 View Code](./01-rust-fundamentals/examples/14-borrowing/) |
| **15** | **Panic Handling** | Unrecoverable errors and the `panic!` macro | [📂 View Code](./01-rust-fundamentals/examples/15-panic/) |
| **16** | **Error Handling** | Recoverable errors with `Result<T, E>` and `match` | [📂 View Code](./01-rust-fundamentals/examples/16-error-handling/) |
| **17** | **Data Structures** | Custom structs and data aggregation basics | [📂 View Code](./01-rust-fundamentals/examples/17-data-structures/) |
| 🧪 | **Practice Project Lab** | CLI File Reader Application with arguments & error handling | [📄 Lab Details](./01-rust-fundamentals/lab.md) |

---

## 📦 Module 02: Structs, Types, and Enums

| # | Topic | Concepts Covered | Source Code |
|:---:|:---|:---|:---:|
| **01** | **Defining Structs** | Defining custom data structures, fields, and types | [📂 View Code](./02-rust-structs-types-enums/examples/1-defining-structs/) |
| **02** | **Creating Structs** | Instantiating structs, initializing field values | [📂 View Code](./02-rust-structs-types-enums/examples/2-creating-structs/) |
| **03** | **Associated Functions** | Constructor pattern (`new()`), static associated methods | [📂 View Code](./02-rust-structs-types-enums/examples/3-associated-functions/) |
| **04** | **Methods & `&self`** | Implementing methods on structs with `&self` and `&mut self` | [📂 View Code](./02-rust-structs-types-enums/examples/4-methods/) |
| **05** | **Debug Attribute** | `#[derive(Debug)]`, custom formatting, pretty printing | [📂 View Code](./02-rust-structs-types-enums/examples/5-more-structs/) |
| **06** | **Strings (`String` vs `&str`)** | Heap-allocated `String` vs string slices (`&str`) | [📂 View Code](./02-rust-structs-types-enums/examples/6-strings/) |
| **07** | **String Manipulation** | Mutating strings, pushing characters, concatenation | [📂 View Code](./02-rust-structs-types-enums/examples/7-mut-string/) |
| **08** | **Basics of Vectors** | Dynamic arrays with `Vec<T>`, initialization macros | [📂 View Code](./02-rust-structs-types-enums/examples/8-vectors/) |
| **09** | **Vector Values** | Reading elements, index safety, handling bounds | [📂 View Code](./02-rust-structs-types-enums/examples/9-vector-values/) |
| **10** | **Vector Elements** | Adding, removing, and iterating over vector items | [📂 View Code](./02-rust-structs-types-enums/examples/10-vector-elements/) |
| **11** | **Defining Enums** | Custom enumerations, classifying categorical data | [📂 View Code](./02-rust-structs-types-enums/examples/11-enums/) |
| **12** | **Enum Types** | Enums with associated data types and payloads | [📂 View Code](./02-rust-structs-types-enums/examples/12-enum-types/) |
| **13** | **Option Enum** | Null safety with `Option<T>` (`Some` vs `None`) | [📂 View Code](./02-rust-structs-types-enums/examples/13-option-enum/) |
| **14** | **Match with Enums** | Pattern matching on enum variants, extracting values | [📂 View Code](./02-rust-structs-types-enums/examples/14-match-enums/) |
| **15** | **Enums in Vectors** | Storing heterogeneous enum variants inside vectors | [📂 View Code](./02-rust-structs-types-enums/examples/15-enums-vectors/) |
| **16** | **Exhaustive Matching** | Handling all enum branches, avoiding wildcard bugs | [📂 View Code](./02-rust-structs-types-enums/examples/16-exhaustive/) |
| 🧪 | **Practice Project Lab** | Structured data application with custom types & error handling | [📄 Lab Details](./02-rust-structs-types-enums/lab.md) |

---

## 📦 Module 03: Applied Rust

| # | Topic | Concepts Covered | Source Code |
|:---:|:---|:---|:---:|
| **01** | **Library Architecture** | Creating Rust libraries with `cargo new --lib`, library structure | [📂 View Code](./03-applied-rust/examples/cli-utils/src/lib.rs) |
| **02** | **Code Documentation** | In-source documentation comments (`///`), documentation tests | [📂 View Code](./03-applied-rust/examples/cli-utils/src/lib.rs) |
| **03** | **Modularization** | Splitting libraries into sub-modules (`colors.rs`, `config.rs`) | [📂 View Code](./03-applied-rust/examples/cli-utils/src/) |
| **04** | **Visibility & Privacy** | Public vs private modules, functions, and struct fields (`pub`) | [📂 View Code](./03-applied-rust/examples/cli-utils/src/config.rs) |
| **05** | **Testing Suite** | Unit tests, integration tests (`tests/`), assert macros | [📂 View Code](./03-applied-rust/examples/cli-utils/tests/) |
| **06** | **Build Automation** | Using a `Makefile` to streamline build, test, and format pipelines | [📂 View Code](./03-applied-rust/examples/cli-utils/Makefile) |
| 🧪 | **Practice Project Lab** | Creating a reusable Rust Library project using Cargo | [📄 Lab Details](./03-applied-rust/lab.md) |

---

## 🚀 Getting Started

### Prerequisites
Install the official Rust toolchain:
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
Navigate to any desired exercise directory and use Cargo to compile, test, and run:

```bash
# Example 1: Run Control Flow from Module 1
cd 01-rust-fundamentals/examples/3-control-flow/conditionals
cargo run

# Example 2: Run Defining Structs from Module 2
cd 02-rust-structs-types-enums/examples/1-defining-structs/defining
cargo run

# Example 3: Test the CLI Utils Library from Module 3
cd 03-applied-rust/examples/cli-utils
cargo test
```

---

## 🙏 Credits & Acknowledgments
The course structure, original starter templates, and curriculum are part of the **Rust Programming Bootcamp Specialization** offered by **Duke University** on Coursera:
* **Course Instructor:** [Alfredo Deza](https://github.com/alfredodeza)
* **Reference Course Repositories:**
  * [alfredodeza/rust-fundamentals](https://github.com/alfredodeza/rust-fundamentals)
  * [alfredodeza/rust-structs-types-enums](https://github.com/alfredodeza/rust-structs-types-enums)
  * [alfredodeza/applied-rust](https://github.com/alfredodeza/applied-rust)

---

<p align="center">
  <b>Developed by 💜 h7</b>
</p>
