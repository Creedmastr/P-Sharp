pub fn print_content(content: String) -> String {
    let content_to_print = content.replace("print(", "").replace(")", "");

    let mut result = String::new();

    let _ = result.push_str(r#"print!("{:#?}", "#);
    let _ = result.push_str(&content_to_print);
    let _ = result.push_str(");\n");

    return result;
}
