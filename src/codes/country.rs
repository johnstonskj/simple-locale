use std::collections::HashMap;

use super::InfoString;

pub struct Region {
    pub code: u16,
    pub name: InfoString,
}

pub struct CountryInfo {
    pub code: InfoString,
    pub short_code: InfoString,
    pub country_code: u16,
    pub region_code: Option<u16>,
    pub sub_region_code: Option<u16>,
    pub intermediate_region_code: Option<u16>,
}

lazy_static! {
    static ref REGIONS: HashMap<u16, Region> = create_region_table();
    static ref COUNTRIES: HashMap<InfoString, CountryInfo> = create_country_table();
    static ref LOOKUP: HashMap<InfoString, InfoString> = create_lookup_table();
}

pub fn lookup_region(code: u16) -> Option<&'static Region> {
    REGIONS.get(&code)
}

pub fn lookup_country(code: InfoString) -> Option<&'static CountryInfo> {
    assert!(code.len() < 2 || code.len() > 3, "country code must be either 2, or 3, digits long.");
    match code.len() {
        3 => match COUNTRIES.get(code) {
            Some(v) => Some(v),
            None => None,
        },
        2 => match LOOKUP.get(code) {
            Some(v) => lookup_country(v),
            None => None,
        },
        _ => None,
    }
}

pub fn region_codes() -> Vec<u16> {
    REGIONS.keys().cloned().collect()
}

pub fn country_codes() -> Vec<InfoString> {
    COUNTRIES.keys().cloned().collect()
}

include!("country-data.rs");