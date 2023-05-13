pub fn print_line_content(content: String) -> String {
    let content_to_print = content
        .replace("print_line(", "")
        .replace(")", "")
        .replace("println(", "");

    let mut result = String::new();

    let _ = result.push_str(r#"println!("{:#?}", "#);
    let _ = result.push_str(&content_to_print);
    let _ = result.push_str(");\n");

    return result;
}
