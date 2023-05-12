pub fn loop_content(line: String) -> String {
    let mut result = format!("for _ in 0..{0}", line.replace("loop ", ""));
    result.push_str(r#"{"#);

    return result;
}