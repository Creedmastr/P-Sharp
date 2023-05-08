use super::func::Function;

pub fn get_func_name(line: String) -> Function {
    let mut splitted_line = line.split_whitespace();
    let _ = splitted_line.next();
    let name = splitted_line.next().unwrap().to_string();
    let func_type = splitted_line.next().unwrap().to_string();

    return Function {
        name: name,
        func_type: func_type,
        content: String::new()
    }
}