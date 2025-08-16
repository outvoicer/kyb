pub fn normalize_string(term: &String) -> String {
    term.chars()
        .flat_map(|c| match c {
            'ā' | 'Ā' => 'a'.to_lowercase(),
            'č' | 'Č' => 'c'.to_lowercase(),
            'ē' | 'Ē' => 'e'.to_lowercase(),
            'ģ' | 'Ģ' => 'g'.to_lowercase(),
            'ī' | 'Ī' => 'i'.to_lowercase(),
            'ķ' | 'Ķ' => 'k'.to_lowercase(),
            'ļ' | 'Ļ' => 'l'.to_lowercase(),
            'ņ' | 'Ņ' => 'n'.to_lowercase(),
            'š' | 'Š' => 's'.to_lowercase(),
            'ū' | 'Ū' => 'u'.to_lowercase(),
            'ž' | 'Ž' => 'z'.to_lowercase(),
            _ => c.to_lowercase(),
        })
        .collect()
}
