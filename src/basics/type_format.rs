pub fn type_format(t: String) -> String {
    match t.as_str() {
        "int" => String::from("i32"),

        "uint" => String::from("u32"),

        "f64" => String::from("f64"),

        "string" => String::from("String"),

        _ => String::new(),
    }
}
