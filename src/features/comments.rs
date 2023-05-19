pub fn remove_comment(line: &String) -> String {
    if line.contains("# ") {
        let result: Vec<&str> = line.split("# ").collect();
        
        return result[0].to_string();
        
    } else {
        return line.to_string();
    }
}