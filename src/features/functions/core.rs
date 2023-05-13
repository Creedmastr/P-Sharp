use crate::features::type_format::{type_format, TypeFormatting};

use super::parser::get_func_name;

pub fn function_content(line: String) -> String {
    let func = get_func_name(line);

    let mut formatted;
    if func.name.contains("(") && func.name.contains(")") {
        formatted = if func.func_type == "null" || func.func_type == "void" {
            format!("fn {0} ", func.name.type_format_in_string())
        } else {
            format!(
                "fn {0} -> {1} ",
                func.name.type_format_in_string(),
                type_format(func.func_type)
            )
        };
    } else {
        formatted = if func.func_type != "null" || func.func_type != "void" {
            format!(
                "fn {0}() -> {1} ",
                func.name.type_format_in_string(),
                type_format(func.func_type)
            )
        } else {
            format!("fn {0}() ", func.name.type_format_in_string())
        };
    }

    formatted.push_str(r#"{"#);
    formatted.push_str("\n");

    return formatted;
}
