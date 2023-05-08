pub fn if_content(line: String) -> String {
    let mut result = line.replace(" & ", " && ");

    result.push_str("{\n");

    return result;
}