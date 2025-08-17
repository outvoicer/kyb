/// Remove SIA from beginning of search
pub fn clean_company_name(input_name: &str) -> &str {
    let prefix = "SIA ";
    if input_name.starts_with(prefix) {
        &input_name[prefix.len()..]
    } else {
        input_name
    }
}
