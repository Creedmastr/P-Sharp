pub fn eflush() -> String{
    return String::from("let _ = std::io::Write::flush(&mut std::io::stderr());\n")
}