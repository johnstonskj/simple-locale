/*!
Character sets registered with IANA.

These are the official names for character sets that may be used in
the Internet and may be referred to in Internet documentation.  These
names are expressed in ANSI_X3.4-1968 which is commonly called
US-ASCII or simply ASCII.  The character set most commonly use in the
Internet and used especially in protocol standards is US-ASCII, this
is strongly encouraged.  The use of the name US-ASCII is also
encouraged.

## Source - IANA

The data used here is taken from the tables in the html page
[IANA](https://www.iana.org/assignments/character-sets/character-sets.xhtml).

**See also**

* [RFC-2978](https://tools.ietf.org/html/rfc2978) IANA Charset
Registration Procedures.
*/

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct CodesetInfo {
    name: String,
    also_known_as: Vec<String>,
    mib_code: u32,
    source: Option<String>,
    references: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref CODESETS: HashMap<String, CodesetInfo> = codesets_from_json();
}

pub fn lookup(name: &str) -> Option<&'static CodesetInfo> {
    assert!(name.len() > 0, "codeset name may not be empty");
    CODESETS.get(name)
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn codesets_from_json() -> HashMap<String, CodesetInfo> {
    info!("codesets_from_json - loading JSON");
    let raw_data = include_bytes!("data/codesets.json");
    let codeset_map: HashMap<String, CodesetInfo> = serde_json::from_slice(raw_data).unwrap();
    info!("codesets_from_json - loaded {} codesets", codeset_map.len());
    codeset_map
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_good_codeset_code() {
        match lookup("UTF-8") {
            None => panic!("was expecting a codeset"),
            Some(codeset) => assert_eq!(codeset.name.to_string(), "UTF-8".to_string()),
        }
    }

    #[test]
    fn test_bad_codeset_code() {
        match lookup(&"UTF-99") {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }
}
