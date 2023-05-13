pub fn error_print_line_content(content: String) -> String {
    let content_to_print = content
        .replace("error_print_line(", "")
        .replace(")", "")
        .replace("err_println(", "");

    let mut result = String::new();

    let _ = result.push_str(r#"eprintln!("{:#?}", "#);
    let _ = result.push_str(&content_to_print);
    let _ = result.push_str(");\n");

    return result;
}
