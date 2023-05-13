pub fn error_print_content(content: String) -> String {
    let content_to_print = content.replace("error_print(", "").replace(")", "").replace("err_print(", "");

    let mut result = String::new();
    
        let _ = result.push_str(r#"eprint!("{:#?}", "#);
        let _ = result.push_str(&content_to_print);
        let _ = result.push_str(");\n");
    

    return result;
}
