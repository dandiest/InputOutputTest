# Rust Interactive CLI Input-Output Test 🦀💰

A lightweight command-line interface (CLI) application built with **Rust** to test input-output functions. This project was developed to master the fundamentals of Rust's memory management and user interaction.

## ⚠️ Disclaimer
> **Educational Purpose:** This project is a dedicated space for mastering the Rust ecosystem. The architectural choices documented here represent an ongoing learning curve, moving from foundational logic to production-grade software engineering patterns.

## ✨ Features
- **Clean Input Handling**: Uses a dedicated helper function to sanitize user input.
- **Infinite Interaction Loop**: Keeps the program running until the user explicitly decides to exit.
- **Lowercase Normalization**: Commands are case-insensitive for a better user experience.

## 🛠️ Technical Implementation
- **String Manipulation**: Implemented `.trim()` and `.to_lowercase()` to handle raw `stdin` buffers.
- **Error Handling**: Used `.expect()` for robust terminal input reading.
- **Modular Design**: Separated the input logic into `create_and_clean()` to keep the `main` function readable.

## 🚀 How to Run
1. Clone the repository:
   `git clone https://github.com/your-username/your-repo-name.git`
2. Navigate to the folder:
   `cd your-repo-name`
3. Run the app:
   `cargo run`
