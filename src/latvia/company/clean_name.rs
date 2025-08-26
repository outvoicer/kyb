/// Remove SIA from beginning of search
pub fn clean_company_name(input_name: &str) -> &str {
    let prefixes = vec!["SIA ", "AS "];

    for prefix in &prefixes {
        if input_name.starts_with(prefix) {
            return &input_name[prefix.len()..].trim();
        }
    }

    input_name.trim()
}
