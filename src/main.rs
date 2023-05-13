use std::{
    fs,
    io::{BufRead, BufReader},
    ops::Not,
    process::Command,
};

use features::{
    commands::{clear::clear_cmd, cmd::cmd_run_command_content},
    functions::core::function_content,
    ifs::{self, elseif::else_if_content},
    out::{
        outputs::{stderr::eflush, stdout::flush},
        print::{
            error_print::error_print_content, error_print_line::error_print_line_content,
            print::print_content, print_line::print_line_content,
        },
    },
};
use string::operations::{push::push_content, remove::remove_content};
use variables::{conversions::into::into_content, var_parser::parse_variable, variable::CanBeType};

use filesystem::{open, write::file_write_content};

mod features;
mod filesystem;
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
    let mut is_in_loop = false;

    for line in buffreader.lines() {
        let line = line.unwrap();

        if line.is_empty() {
            final_file_content.push_str("\n");
            continue;
        }

        if line.contains("var") {
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
                } => final_file_content
                    .push_str(&format!("let {0}: f64 = {1};\n", var.name, var.content)),

                _ => {
                    let new_content = match var.content_quoted {
                        x if x.contains("remove(") => remove_content(x),

                        x if x.contains("push(") => push_content(x),

                        x if x.contains("fs:file_to_string(") => {
                            open::file_open_content(var.content)
                        }

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
            // Checks specific for the functions
            if is_in_function {
                if line == "endfunc" {
                    is_in_function = false;
                    final_file_content.push_str(r#"}"#);
                    continue;
                }

                final_file_content.push_str(r#"     "#);

                if line.contains("(").not()
                    && line.contains(")").not()
                    && line.split_whitespace().count() == 1
                {
                    final_file_content.push_str(&format!("{}\n", line));
                    continue;
                }
            }

            if is_in_loop {
                if line == "endloop" {
                    is_in_loop = false;
                    final_file_content.push_str(r#"}"#);
                    continue;
                }

                final_file_content.push_str(r#"     "#);
            }

            // For the ifs
            if line == "endif" {
                final_file_content.push_str(r#"}"#);
                continue;
            }

            // Get the final content
            let new_content = match line {
                // All the prints
                // Uses not error_, etc. to not detect error_print as print & error_print_line as print_line
                x if x.contains("print(")
                    && x.contains("error_print(").not()
                    && x.contains("err_print(").not() =>
                {
                    print_content(x)
                }

                x if x.contains("print_line(") | x.contains("println")
                    && x.contains("error_print_line(").not()
                    && x.contains("err_println(").not() =>
                {
                    print_line_content(x)
                }

                x if x.contains("error_print(") | x.contains("err_print(") => {
                    error_print_content(x)
                }

                x if x.contains("error_print_line(") | x.contains("err_println(") => {
                    error_print_line_content(x)
                }

                // Functions
                x if x.contains("function ") | x.contains("") => {
                    is_in_function = true;
                    function_content(x)
                }

                x if x == "loop" => {
                    is_in_loop = true;
                    features::functions::loops::infinite_loops_content()
                }

                x if x.contains("loop ") => {
                    is_in_loop = true;
                    features::functions::loops::define_loops_content(x)
                }

                x if x.contains("cmd:run_command(") => cmd_run_command_content(x),

                // Ifs
                x if x.starts_with("if ") => ifs::ifs::if_content(x),

                x if x == "else" => ifs::elses::else_content(x),

                x if x.starts_with("else if") => else_if_content(x),

                // Utilitaries
                x if x.contains("fs:write_file(") => file_write_content(x),

                x if x == "out:flush()" => flush(),

                x if x == "eout:flush()" => eflush(),

                // Maths
                x if x.contains(" += ") => features::maths::plusequal::plusequal_content(x),

                // Kinda macro functions
                x if x.contains("cmd:clear_cmd(") => clear_cmd(),

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
