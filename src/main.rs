use std::{
    fs,
    io::{BufRead, BufReader},
    ops::Not,
};

use variables::{var_parser::parse_variable, variable::CanBeType};

mod variables;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args[1].ends_with(".ps").not() && args[1].ends_with("/").not() {
        panic!("The argument specified is nor a valid file nor a valid directory!")
    }

    let file = fs::File::open(&args[1]).unwrap();
    let buffreader = BufReader::new(file);
    let mut final_file_content = String::new();

    for line in buffreader.lines() {
        let line = line.unwrap();

        if line.starts_with("var") {
            let var = parse_variable(line.clone());

            match var.can_be_type {
                CanBeType {
                    can_be_int: true,
                    can_be_uint: false,
                    can_be_float: false,
                } => final_file_content.push_str(
                    format!(
                        "let {0}: i32 = {1}",
                        var.name,
                        var.content.parse::<i32>().unwrap()
                    )
                    .as_str(),
                ),

                CanBeType {
                    can_be_int: true,
                    can_be_uint: true,
                    can_be_float: false,
                } => final_file_content.push_str(
                    format!(
                        "let {0}: u32 = {1}",
                        var.name,
                        var.content.parse::<u32>().unwrap()
                    )
                    .as_str(),
                ),

                CanBeType {
                    can_be_int: false,
                    can_be_uint: false,
                    can_be_float: true,
                } => final_file_content.push_str(
                    format!(
                        "let {0}: f64 = {1}",
                        var.name,
                        var.content.parse::<f64>().unwrap()
                    )
                    .as_str(),
                ),

                CanBeType {
                    can_be_int: false,
                    can_be_uint: false,
                    can_be_float: false,
                } => final_file_content
                    .push_str(format!("let {0} = {1};\n", var.name, var.content_quoted).as_str()),

                _ => final_file_content
                    .push_str(format!("let {0} = {1};\n", var.name, var.content_quoted).as_str()),
            }
        }
    }

    eprintln!("{}", final_file_content);
}
