/* generated from ISO-639 data files */

fn create_lookup_table() -> HashMap<String, LanguageInfo> {
    let mut table = HashMap::new();
    table.insert("aab".to_string(), LanguageInfo {
        code: "aab".to_string(),
        reference_name: "Alumu-Tesu".to_string(),
        indigenous_name: None,
        other_names: None,
        bibliographic_code: None,
        terminology_code: None,
        short_code: None,
        scope: LanguageScope::Individual,
        l_type: LanguageType::Living,
        family_members: None,
    });
    table
}

fn create_id_lookup_table() -> HashMap<String, String> {
    let mut table = HashMap::new();
    table.insert("aa".to_string(), "aab".to_string());
    table
}
