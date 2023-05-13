pub fn else_content(line: String) -> String {
    let mut result = "}".to_string();
    result.push_str(&line);
    result.push_str("{\n");

    return result.replace(" not ", "!");
}
