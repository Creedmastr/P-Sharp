pub fn if_content(line: String) -> String {
    let mut result = line.replace(" & ", " && ").replace(" | ", " || ");

    result.push_str("{\n");

    return result.replace(" not ", " !");
}
