use std::{
    fs,
    io::{BufRead, BufReader},
    ops::Not,
    process::Command,
};

use basics::{
    functions::parser::get_func_name,
    ifs::{self, elseif::else_if_content},
    out::{
        print::{
            error_print::error_print_content, error_print_line::error_print_line_content,
            print::print_content, print_line::print_line_content,
        },
        std::{stderr::eflush, stdout::flush},
    },
};
use string::operations::{push::push_content, remove::remove_content};
use variables::{
    conversions::into::into_content,
    filesystem::{open, write::file_write_content},
    var_parser::parse_variable,
    variable::CanBeType,
};

use crate::basics::type_format::{type_format, TypeFormatting};

mod basics;
mod makefile;
mod string;
mod variables;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args[1].ends_with(".ps").not() && args[1].ends_with("/").not() {
        panic!("The argument specified is nor a valid file nor a valid directory!")
    }

    // Get the lines
    let file = fs::File::open(&args[1]).unwrap();
    let buffreader = BufReader::new(file);
    let mut final_file_content = String::new();

    let mut is_in_function = false;

    for line in buffreader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            final_file_content.push_str("\n");
            continue;
        }

        if line.starts_with("var") {
            let var = parse_variable(line.clone());

            // Put the right type and not an inferred one if possible
            match var.can_be_type {
                CanBeType {
                    can_be_int: true,
                    can_be_uint: false,
                    can_be_float: true,
                } => final_file_content.push_str(&format!(
                    "let {0}: i32 = {1}; \n",
                    var.name,
                    var.content.parse::<i32>().unwrap()
                )),

                CanBeType {
                    can_be_int: true,
                    can_be_uint: true,
                    can_be_float: true,
                } => final_file_content.push_str(&format!(
                    "let {0}: u32 = {1}; \n",
                    var.name,
                    var.content.parse::<u32>().unwrap()
                )),

                CanBeType {
                    can_be_int: false,
                    can_be_uint: false,
                    can_be_float: true,
                } => final_file_content.push_str(&format!(
                    "let {0}: f64 = {1};\n",
                    var.name,
                    var.content.parse::<f64>().unwrap()
                )),

                _ => {
                    let new_content = match var.content_quoted {
                        x if x.contains("remove(") => remove_content(x),

                        x if x.contains("push(") => push_content(x),

                        x if x.contains("file_to_string(") => open::file_open_content(var.content),

                        x if x.contains("into(") => into_content(var.content),

                        _ => var.content_quoted.clone(),
                    };

                    final_file_content.push_str(&format!(
                        "let {0} = {1};\n",
                        var.name,
                        new_content.replace("mutable", "")
                    ))
                }
            }
        } else {
            let mut new_content = String::new();

            // Checks specific for the functions
            if is_in_function {
                if line.contains("endfunc") {
                    is_in_function = false;
                    final_file_content.push_str(r#"}"#);
                    continue;
                }

                final_file_content.push_str(r#"     "#);

                if line.contains("(").not()
                    && line.contains(")").not()
                    && line.split_whitespace().count() == 1
                {
                    new_content.push_str(&format!("{}\n", line));
                    continue;
                }
            }

            // For the ifs
            if line == "endif" {
                final_file_content.push_str(r#"}"#);
                continue;
            }

            // Get the final content
            new_content = match line {
                // All the prints
                // Uses not error_, etc. to not detect error_print as print & error_print_line as print_line
                x if x.contains("print(") && x.contains("error_print(").not() => print_content(x),

                x if x.contains("print_line(") && x.contains("error_print_line(").not() => {
                    print_line_content(x)
                }

                x if x.contains("error_print(") => error_print_content(x),

                x if x.contains("error_print_line(") => error_print_line_content(x),

                x if x.contains("function ") => {
                    let func = get_func_name(x);

                    is_in_function = true;
                    let mut formatted;
                    if func.name.contains("(") && func.name.contains(")") {
                        formatted = if func.func_type == "null" || func.func_type == "void" {
                            format!("fn {0} ", func.name.type_format_in_string())
                        } else {
                            format!("fn {0} -> {1} ", func.name, type_format(func.func_type))
                        };
                    } else {
                        formatted = if func.func_type != "null" || func.func_type != "void" {
                            format!("fn {0}() -> {1} ", func.name, type_format(func.func_type))
                        } else {
                            format!("fn {0}() ", func.name)
                        };
                    }

                    formatted.push_str(r#"{"#);
                    formatted.push_str("\n");

                    formatted
                }

                x if x.starts_with("if ") => ifs::ifs::if_content(x),

                x if x == "else" => ifs::elses::else_content(x),

                x if x.starts_with("else if") => else_if_content(x),

                x if x.contains("write_file(") => file_write_content(x),

                x if x == "flush()" => flush(),

                x if x == "eflush()" => eflush(),

                _ => line,
            };

            final_file_content.push_str(&format!("{}", new_content.replace("mutable", "mut")))
        }
    }

    // Check the args, by first checking the length not to get errors
    match args.len() {
        x if x >= 3 => {
            makefile::build::build_file(
                final_file_content,
                args[2].parse::<bool>().unwrap_or(false),
            );

            if args[2] == "run" {
                let _ = Command::new("bash").arg("./a.out");
            }
        }

        _ => makefile::build::build_file(final_file_content, false),
    }
}
