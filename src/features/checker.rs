#[derive(Clone, Copy)]
pub struct Modifiers {
    pub allow_rust_expressions: bool,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers {
            allow_rust_expressions: false,
        }
    }
}

pub fn check_modifier(line: &String, current_modifier: Modifiers) -> (Modifiers, String) {
    let mut new_modifiers = current_modifier;

    if line.starts_with("#![") {
        if line == "#![allow(rust_expressions)]" {
            new_modifiers.allow_rust_expressions = true;
            return (new_modifiers, String::new());
        }
    }

    return (new_modifiers, line.clone());
}

pub fn check(line: &String, line_counter: u32, current_modifier: Modifiers) {
    if line.ends_with(";") {
        panic!("ERROR: Semicolon misplaced on line {}", line_counter);
    };

    if !current_modifier.allow_rust_expressions {
        if line.contains("Command::new(") {
            panic!("ERROR: 'Command::new(' is a invalid Rust expression. Try using cmd:run_command(<commmand>) on line {}", line_counter);
        };

        if line.contains("fs::File::") {
            panic!(
                "ERROR: 'fs::File::' is a invalid Rust expression. Try using fs: on line {}",
                line_counter
            );
        };
    };
}
