use std::{fs, process::Command};

pub fn build_file(file_content: String, is_rust: bool) {
    let mut final_file = String::new();
    // Headers
    let _ = final_file.push_str("#![allow(dead_code)]\n#![allow(non_snake_case)]\n#![allow(non_ascii_idents)]\n#![allow(unused_variables)]\n \n");

    // Main content
    let _ = final_file.push_str("fn main() {\n");
    let _ = final_file.push_str(&file_content);
    let _ = final_file.push_str("\n}");

    fs::write("./a.rs", final_file).expect("Couldn't write temp file");
    if !is_rust {
        let out = Command::new("rustc").arg("./a.rs").output().unwrap();
        let _ = fs::remove_file("./a.rs");
        eprintln!("{:#?}", out)
    };
}
