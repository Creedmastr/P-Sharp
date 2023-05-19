pub fn remove_comment(line: &String) -> String {
    if line.contains("# ") {
        let result: Vec<&str> = line.split("# ").collect();
        // TODO: Only take the last one as a comment
        return result[0].to_string();
        
    } else {
        return line.to_string();
    }
}