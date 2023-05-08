use super::variable::{CanBeType, Variable};

pub fn parse_variable(line: String) -> Variable {
    let current_line = line.clone();
    let _ = current_line.replace("var ", "");

    let mut splitted = current_line.split_whitespace();
    let _name = splitted.next().unwrap().to_string();
    let name = splitted.next().unwrap().to_string();

    let content_quoted = current_line.replace("=", "").replace("var ", "").replace(format!("{0}  ", name).as_str(), "");

    let temp_var = Variable {
        name: name,
        content: content_quoted.replace(r#"""#, ""),
        content_quoted: content_quoted,
        can_be_type: CanBeType::default(),
    };

    let can_be_type = {
        let can_be_int = match temp_var.content.parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        };

        let can_be_uint = match temp_var.content.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false,
        };

        let can_be_float = match temp_var.content.parse::<f64>() {
            Ok(_) => true,
            Err(_) => false,
        };

        CanBeType {
            can_be_int,
            can_be_uint,
            can_be_float,
        }
    };

    let final_var = Variable {
        name: temp_var.name,
        content: temp_var.content,
        content_quoted: temp_var.content_quoted,
        can_be_type: can_be_type,
    };

    return final_var;
}