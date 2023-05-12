pub fn push_stringed_vector(stringed_vec: String, split_char: &str) -> String {
    let sliced_vector = stringed_vec.split(split_char).collect::<Vec<&str>>();
    let mut result: String = String::new();

    result.push_str("[");

    for item in sliced_vector {
        result.push_str(&item);
        result.push(',');
    }

    result.push_str("]");

    return result;
}