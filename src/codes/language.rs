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
    static ref LANGUAGES: HashMap<String, LanguageInfo> = create_lookup_table();
    static ref LOOKUP: HashMap<String, String> = create_id_lookup_table();
}

pub fn lookup(code: &str) -> Option<&'static LanguageInfo> {
    println!(">>> {}", code);
    assert!(code.len() == 2 || code.len() == 3, "language code must be either 2, or 3, characters long.");
    match code.len() {
        3 => match LANGUAGES.get(code) {
            Some(v) => Some(v),
            None => None,
        },
        2 => match LOOKUP.get(code) {
            Some(v) => {
                assert_eq!(v.len(), 3);
                println!("rec: {}", v);
                match LANGUAGES.get(code) {
                    Some(v) => Some(v),
                    None => None,
                }
            },
            None => None,
        },
        _ => None,
    }
}

pub fn language_codes() -> Vec<String> {
    LANGUAGES.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

include!("language-data.rs");

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
        println!("*****");
        match lookup("aab") {
            None => println!("OH CRAP"),
            Some(l) => println!("{:#?}", to_string_pretty(l)),
        }
    }
}