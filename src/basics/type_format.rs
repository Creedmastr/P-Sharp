pub fn type_format(t: String) -> String {
    match t.as_str() {
        "int" => String::from("i32"),

        "uint" => String::from("u32"),

        "float" => String::from("f64"),

        "string" => String::from("String"),

        _ => String::new(),
    }
}

pub trait TypeFormatting {
    fn type_format_in_string(&self) -> String;
}

impl TypeFormatting for String {
    fn type_format_in_string(&self) -> String {
        self.replace("uint", "u32")
            .replace("int", "i32")
            .replace("float", "f64")
            .replace("string", "String")
    }
}
