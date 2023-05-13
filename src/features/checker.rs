struct Modifiers {
    allow_rust_expressions: bool,
}

impl Modifiers {
    fn new() -> Modifiers {
        Modifiers { allow_rust_expressions: false }
    }
}

pub fn check(line: &String, line_counter: u32) {
    let mut modifiers = Modifiers::new();

    if line_counter == 1 && line.starts_with("#![") {
        if line == "#![allow(rust_expressions)]" {
            modifiers.allow_rust_expressions = true;
        }
    } else {
        if line.ends_with(";") {
            panic!(
                "ERROR: Semicolon placed on the wrong spot on line {}",
                line_counter
            );
        };
    
        if line.contains("Commands::new(") {
            panic!("ERROR: 'Commands::new(' is a invalid Rust expression. Try using cmd:run_command(<commmand>) on line {}", line_counter);
        };
    };
}
