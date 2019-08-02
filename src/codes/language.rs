use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub enum LanguageScope {
    Individual,
    MacroLanguage,
    Special,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LanguageType {
    Ancient,
    Constructed,
    Extinct,
    Historical,
    Living,
    Special,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LanguageInfo {
    pub code: String,
    pub reference_name: String,
    pub indigenous_name: Option<String>,
    pub other_names: Option<Vec<String>>,
    pub bibliographic_code: Option<String>,
    pub terminology_code: Option<String>,
    pub short_code: Option<String>,
    pub scope: LanguageScope,
    pub l_type: LanguageType,
    pub family_members: Option<Vec<String>>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref LANGUAGES: HashMap<String, LanguageInfo> = languages_from_json();
    static ref LOOKUP: HashMap<String, String> = load_language_lookup();
}

pub fn lookup(code: &str) -> Option<&'static LanguageInfo> {
    debug!("language::lookup {}", code);
    assert!(code.len() == 2 || code.len() == 3, "language code must be either 2, or 3, characters long.");
    match code.len() {
        3 =>
            match LANGUAGES.get(code) {
                Some(v) => Some(v),
                None => None,
            }
        ,
        2 =>
            match LOOKUP.get(code) {
                Some(v) => {
                    debug!("language::lookup {} -> {}", code, v);
                    lookup(v)
                },
                None => None,
            }
        ,
        _ => None,
    }
}

pub fn language_codes() -> Vec<String> {
    LANGUAGES.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn languages_from_json() -> HashMap<String, LanguageInfo> {
    info!("languages_from_json - loading JSON");
    let raw_data = include_bytes!("data/languages.json");
    let language_map: HashMap<String, LanguageInfo> = serde_json::from_slice(raw_data).unwrap();
    info!("languages_from_json - loaded {} countries", language_map.len());
    language_map
}

fn load_language_lookup() -> HashMap<String, String> {
    info!("load_language_lookup - create from COUNTRIES");
    let mut lookup_map: HashMap<String, String> = HashMap::new();
    for language in LANGUAGES.values() {
        if let Some(short_code) = &language.short_code {
            lookup_map.insert(short_code.to_string(), language.code.to_string());
        }
    }
    info!("load_language_lookup - mapped {} countries", lookup_map.len());
    lookup_map
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::ser::to_string_pretty;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_language_loading() {
        match lookup("aab") {
            None => println!("test_language_loading NO 'aab'"),
            Some(l) => println!("test_language_loading {:#?}", to_string_pretty(l)),
        }
    }
}