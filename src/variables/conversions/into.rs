use crate::variables::variable::CanBeType;

pub fn into_content(var_content: String) -> String { 
    let content_quoted = {
        let name = var_content.replace(".into()", "");
        let mut result = String::from(r#"""#);
        result.push_str(&name);
        result.push_str(r#"""#);

        result
    };
    let to_parse = content_quoted.replace(r#"""#, "");

    let can_be_type = {
        let can_be_int = match to_parse.parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        };

        let can_be_uint = match to_parse.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false,
        };

        let can_be_float = match to_parse.parse::<f64>() {
            Ok(_) => true,
            Err(_) => false,
        };

        CanBeType {
            can_be_int,
            can_be_uint,
            can_be_float,
        }
    };

    if can_be_type.can_be_uint {
        return format!("{}.parse::<u32>()", content_quoted);
    } else if can_be_type.can_be_int && !can_be_type.can_be_uint {
        return format!("{}.parse::<i32>()", content_quoted);
    } else if can_be_type.can_be_float && !can_be_type.can_be_int {
        return format!("{}.parse::<f64>()", content_quoted);
    } else {
        return var_content;
    }
}
