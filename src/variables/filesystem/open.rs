pub fn file_open_content(var: String) -> String {
    let file_path = var.replace("file_to_string(", "").replace(")", "");
    let mut result = r#"""#.to_string();

    let _ = result.push_str(&file_path);
    let _ = result.push_str(&r#"""#);
    let new_content = format!("std::fs::read_to_string({0}).unwrap()", result);

    return new_content;
}
