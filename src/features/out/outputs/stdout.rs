pub fn flush() -> String {
    return String::from("let _ = std::io::Write::flush(&mut std::io::stdout());\n");
}
