pub fn remove_content(content: String) -> String {
    let binding = content.replace("remove(", "").replace(")", "");
    let mut content_to_remove = binding.split(".");
    let part1 = content_to_remove.next().unwrap();
    let part2 = content_to_remove.next().unwrap();

    let mut result = String::new();
    let _ = result.push_str(part1);
    let _ = result.push_str(r#".replace("#);
    let _ = result.push_str(part2);
    let _ = result.push_str(r#", "")"#);

    return result;
}
