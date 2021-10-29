pub fn convert_str_to_vec(vec_str: String) -> Vec<String> {
    let removed_str = vec_str.replace(&['[', ']', '"', ' '][..], "");
    removed_str.split(",").map(|v| { String::from(v.trim()) }).collect()
}