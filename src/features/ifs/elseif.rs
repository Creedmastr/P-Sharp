use super::ifs::if_content;

pub fn else_if_content(line: String) -> String {
    let line = line
        .replace("else", "")
        .replace(" & ", " && ")
        .replace(" | ", " || ");

    let second = if_content(line);

    let mut final_str = r#"}"#.to_string();
    final_str.push_str(&format!("else {} ", second));
    final_str.push_str(r#""#);

    return final_str.replace(" not ", "!");
}
