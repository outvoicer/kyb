/// Remove SIA and AS from beginning of search
pub fn clean_company_name(input_name: &str) -> String {
    let prefixes = vec!["SIA ", "AS ", "sia ", "as ", "Sia ", "As "];
    let suffixes = vec![" SIA", " AS", " sia", " as", " Sia", " As"];

    // Remove apostrophes
    // Remove apostrophes
    let mut cleaned_name = input_name.replace("'", "");

    for prefix in &prefixes {
        if cleaned_name.starts_with(prefix) {
            cleaned_name = cleaned_name[prefix.len()..].trim().to_string();
            break;
        }
    }

    for suffix in &suffixes {
        if cleaned_name.ends_with(suffix) {
            cleaned_name = cleaned_name[..cleaned_name.len() - suffix.len()]
                .trim()
                .to_string();
            break;
        }
    }

    cleaned_name.trim().to_string()
}

#[cfg(test)]
mod tests {
    use crate::latvia::company::clean_name::clean_company_name;
    use actix_web::test;

    #[test]
    async fn sia_suffix() {
        // preffix
        let search_term = "SIA Raimond fantastic".to_string();
        let result = clean_company_name(&search_term);
        assert_eq!(result, "Raimond fantastic".to_string(),);
        // suffix
        let search_term = "Raimond fantastic SIA".to_string();
        let result = clean_company_name(&search_term);
        assert_eq!(result, "Raimond fantastic".to_string(),);
        // apostrofe
        let search_term = "SIA 'Raimond fantastic".to_string();
        let result = clean_company_name(&search_term);
        assert_eq!(result, "Raimond fantastic".to_string(),);
    }
}
