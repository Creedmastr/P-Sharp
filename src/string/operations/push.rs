pub fn push_content(line: String) -> String {
    let current_line = line.replace("push(", "");
    let current_line = current_line.replace(")", "");
    let mut splitted = current_line.split(".");

    let first_part = splitted.next().unwrap();
    let second_part = splitted.next().unwrap();

    let mut result = String::new();
    let _ = result.push_str(r#"format!("{}{}", "#);
    let _ = result.push_str(&first_part);
    let _ = result.push_str(",");
    let _ = result.push_str(&second_part);
    let _ = result.push_str(")");

    return result;
}
