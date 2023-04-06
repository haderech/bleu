pub fn convert_str_to_vec(vec_str: String) -> Vec<String> {
	let arranged = vec_str.replace(&['[', ']', '"', ' '][..], "");
	arranged.split(",").map(|v| String::from(v.trim())).collect()
}
