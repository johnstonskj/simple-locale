use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptInfo {
    alphabetic_code: String,
    numeric_code: u16,
    name: String,
    alias: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref SCRIPTS: HashMap<String, ScriptInfo> = scripts_from_json();
    static ref NUMERIC_LOOKUP: HashMap<u16, String> = load_script_lookup();
}

pub fn lookup_by_alpha(alphabetic_code: &str) -> Option<&'static ScriptInfo> {
    assert_eq!(alphabetic_code.len(), 4, "script code is expected to be 3 characters");
    SCRIPTS.get(alphabetic_code)
}

pub fn lookup_by_numeric(numeric_code: &u16) -> Option<&'static ScriptInfo> {
    match NUMERIC_LOOKUP.get(&numeric_code) {
        Some(v) => lookup_by_alpha(v),
        None => None,
    }
}

pub fn script_alpha_codes() -> Vec<String> {
    SCRIPTS.keys().cloned().collect()
}

pub fn script_numeric_codes() -> Vec<u16> {
    NUMERIC_LOOKUP.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn scripts_from_json() -> HashMap<String, ScriptInfo> {
    info!("scripts_from_json - loading JSON");
    let raw_data = include_bytes!("data/scripts.json");
    let script_map: HashMap<String, ScriptInfo> = serde_json::from_slice(raw_data).unwrap();
    info!("scripts_from_json - loaded {} codesets", script_map.len());
    script_map
}

fn load_script_lookup() -> HashMap<u16, String> {
    info!("load_script_lookup - create from SCRIPTS");
    let mut lookup_map: HashMap<u16, String> = HashMap::new();
    for script in SCRIPTS.values() {
        println!("{} -> {}", &script.numeric_code, &script.alphabetic_code);
        lookup_map.insert(script.numeric_code, script.alphabetic_code.to_string());
    }
    info!(
        "load_script_lookup - mapped {} countries",
        lookup_map.len()
    );
    lookup_map
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_good_script_alpha_code() {
        match lookup_by_alpha("Hluw") {
            None => panic!("was expecting a script"),
            Some(script) => {
                assert_eq!(script.alphabetic_code.to_string(), "Hluw".to_string());
                assert_eq!(script.numeric_code, 80);
                //assert_eq!(script.alias.unwrap().to_string(), "Anatolian_Hieroglyphs".to_string())
            },
        }
    }

    #[test]
    fn test_bad_script_alpha_code() {
        match lookup_by_alpha(&"UTF8") {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }

    #[test]
    fn test_good_script_numeric_code() {
        match lookup_by_numeric(&80) {
            None => panic!("was expecting a script"),
            Some(script) => {
                assert_eq!(script.alphabetic_code.to_string(), "Hluw".to_string());
                assert_eq!(script.numeric_code, 80);
                //assert_eq!(script.alias.unwrap().to_string(), "Anatolian_Hieroglyphs".to_string())
            },
        }
    }

    #[test]
    fn test_bad_script_numeric_code() {
        match lookup_by_numeric(&0) {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }

    #[test]
    fn test_script_codes() {
        let codes = script_alpha_codes();
        assert!(codes.len() > 0);
        let numerics = script_numeric_codes();
        assert!(numerics.len() > 0);
    }
}
