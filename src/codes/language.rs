use std::collections::HashMap;

pub enum LanguageScope {
    Individual,
    MacroLanguage,
    Special,
}

pub enum LanguageType {
    Ancient,
    Constructed,
    Extinct,
    Historical,
    Living,
    Special,
}

pub type InfoString = &'static str;

pub struct LanguageInfo {
    pub identifier: InfoString,
    pub reference_name: InfoString,
    pub bibliographic_identifier: Option<InfoString>,
    pub terminology_identifier: Option<InfoString>,
    pub short_identifier: Option<InfoString>,
    pub scope: LanguageScope,
    pub l_type: LanguageType,
    pub family_members: Option<Vec<InfoString>>,
}

lazy_static! {
    static ref PRIMARY: HashMap<InfoString, LanguageInfo> = create_lookup_table();
    static ref SECONDARY: HashMap<InfoString, InfoString> = create_id_lookup_table();
}

pub fn lookup(code: InfoString) -> Option<&'static LanguageInfo> {
    assert!(code.len() < 2 || code.len() > 3, "language code must be either 2, or 3, digits long.");
    match code.len() {
        3 => match PRIMARY.get(code) {
            Some(v) => Some(v),
            None => None,
        },
        2 => match SECONDARY.get(code) {
            Some(v) => lookup(v),
            None => None,
        },
        _ => None,
    }
}

include!("language-data.rs");
