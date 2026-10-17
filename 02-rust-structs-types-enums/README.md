# 🦀 Rust Structs, Types, and Enums

<p align="center">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=for-the-badge&logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Build-Cargo-informational?style=for-the-badge&logo=rust" alt="Cargo" />
  <img src="https://img.shields.io/badge/Course-Duke_University_%2F_Coursera-blue?style=for-the-badge" alt="Course" />
  <img src="https://img.shields.io/badge/Status-Completed-success?style=for-the-badge" alt="Status" />
</p>

A curated collection of hands-on labs, coding exercises, and practical implementations covering **Rust Structs, Types, and Enums** (Rust Bootcamp Week 3 / Duke University Specialization).

---

## 📌 Topics & Exercises Overview

| # | Topic | Concepts Covered | Source Code |
|:---:|:---|:---|:---:|
| **01** | **Defining Structs** | Defining custom data structures, fields, and types | [📂 View Code](./examples/1-defining-structs/) |
| **02** | **Creating Structs** | Instantiating structs, initializing field values | [📂 View Code](./examples/2-creating-structs/) |
| **03** | **Associated Functions** | Constructor pattern (`new()`), static associated methods | [📂 View Code](./examples/3-associated-functions/) |
| **04** | **Methods & `&self`** | Implementing methods on structs with `&self` and `&mut self` | [📂 View Code](./examples/4-methods/) |
| **05** | **Debug Attribute** | `#[derive(Debug)]`, custom formatting, pretty printing | [📂 View Code](./examples/5-more-structs/) |
| **06** | **Strings (`String` vs `&str`)** | Heap-allocated `String` vs string slices (`&str`) | [📂 View Code](./examples/6-strings/) |
| **07** | **String Manipulation** | Mutating strings, pushing characters, concatenation | [📂 View Code](./examples/7-mut-string/) |
| **08** | **Basics of Vectors** | Dynamic arrays with `Vec<T>`, initialization macros | [📂 View Code](./examples/8-vectors/) |
| **09** | **Vector Values** | Reading elements, index safety, handling bounds | [📂 View Code](./examples/9-vector-values/) |
| **10** | **Vector Elements** | Adding, removing, and iterating over vector items | [📂 View Code](./examples/10-vector-elements/) |
| **11** | **Defining Enums** | Custom enumerations, classifying categorical data | [📂 View Code](./examples/11-enums/) |
| **12** | **Enum Types** | Enums with associated data types and payloads | [📂 View Code](./examples/12-enum-types/) |
| **13** | **Option Enum** | Null safety with `Option<T>` (`Some` vs `None`) | [📂 View Code](./examples/13-option-enum/) |
| **14** | **Match with Enums** | Pattern matching on enum variants, extracting values | [📂 View Code](./examples/14-match-enums/) |
| **15** | **Enums in Vectors** | Storing heterogeneous enum variants inside vectors | [📂 View Code](./examples/15-enums-vectors/) |
| **16** | **Exhaustive Matching** | Handling all enum branches, avoiding wildcard bugs | [📂 View Code](./examples/16-exhaustive/) |
| 🧪 | **Practice Project Lab** | Structured data application with custom types & error handling | [📄 Lab Details](./lab.md) |

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
# Example: Running the Defining Structs exercise
cd examples/1-defining-structs/defining

# Check syntax without building
cargo check

# Compile and run
cargo run
```

---

## 🙏 Credits & Acknowledgments
The course structure, original starter templates, and curriculum are part of the **Rust Structs, Types, and Enums Bootcamp** offered by **Duke University** on Coursera:
* **Course Instructor:** [Alfredo Deza](https://github.com/alfredodeza)
* **Reference Coursework:** [alfredodeza/rust-structs-types-enums](https://github.com/alfredodeza/rust-structs-types-enums)

---

## 👤 Author
- **H7 💜** ([@hashan-7](https://github.com/hashan-7))
