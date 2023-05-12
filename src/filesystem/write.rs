pub fn file_write_content(var_content: String) -> String {
    let file_path = var_content.replace("write_file(", "").replace(")", "");
    let mut file_path = file_path.split(", ");

    let mut result = file_path.next().unwrap().to_string();
    let _ = result.push_str(", ");
    let _ = result.push_str(&file_path.next().unwrap().to_string());

    let new_content = format!("std::fs::write({0}).unwrap();\n", result);
    return new_content;
}
