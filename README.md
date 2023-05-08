# About

This project is my attempt at creating a programming language. It is variable driven, meaning that everything should be inside variables. It only supports one function by line. The variables can be rewritten for good memory management or chained methods/functions.

# Download

To use the compiler, just download the binary, or compile it yourself from this `cargo` project.

# Build
To install RustUp (on Linux/Mac):
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
First clone this repo:
`git clone https://github.com/Creedmastr/P-Sharp``
Then cd and build:
`cd P-Sharp && cargo build --release`

# Usage

To:
- Start: create a file named .ps
- Compile it with `psharp ./<filename> <is_rust>`. Is rust is not an optionnal argument, which you can set to true if you want the Rust file before it is compiled. Please note that you have to be in the same directory as the file for everything to work properly.
- Create a function with the `function` keyword, followed by its name (with or without parenthesis) and the type it returns (int, uint, float or string, or any other Rust type [NOT DIRECTLY SUPPORTED]). For no return type, use the `null` or `void` type.
- Create a variable with the var keyword. Types are inferred.
- Use print for print, print_line for print_line, and error_print along with error_print_line for error prints.

Those are the current features for the current version (when I'm writing this). It's not much by I plan to improve it a little.