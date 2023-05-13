pub fn define_loops_content(line: String) -> String {
    let mut result = format!("for _ in 0..{0}", line.replace("loop ", ""));
    result.push_str(r#"{"#);

    return result;
}

pub fn infinite_loops_content() -> String {
    let mut result = "loop".to_string();
    result.push_str(r#"{"#);

    return result;
}
