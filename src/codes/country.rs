use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::InfoString;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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

#[derive(Serialize, Deserialize)]
pub struct RegionS {
    pub code: u16,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CountryInfoS {
    pub code: String,
    pub short_code: String,
    pub country_code: u16,
    pub region_code: Option<u16>,
    pub sub_region_code: Option<u16>,
    pub intermediate_region_code: Option<u16>,
}
// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref REGIONS: HashMap<u16, &'static Region> = create_region_table();
    static ref COUNTRIES: HashMap<InfoString, &'static CountryInfo> = create_country_table();
}

pub fn lookup_region(code: u16) -> Option<&'static Region> {
    match REGIONS.get(&code) {
        Some(v) => Some(*v),
        None => None,
    }
}

pub fn lookup_country(code: &str) -> Option<&'static CountryInfo> {
    println!(">>> lookup_country {}", code);
    assert!(code.len() == 2 || code.len() == 3, "country code must be either 2, or 3, characters long.");
    match COUNTRIES.get(code) {
        Some(v) => Some(v),
        None => None,
    }
}

pub fn region_codes() -> Vec<u16> {
    REGIONS.keys().cloned().collect()
}

pub fn country_codes() -> Vec<InfoString> {
    COUNTRIES.keys().cloned().collect()
}

#[test]
fn test_country_load() {
    let c = CountryInfoS {
        code: "AFG".to_string(),
        short_code: "AF".to_string(),
        country_code: 4,
        region_code: Some(142),
        sub_region_code: Some(34),
        intermediate_region_code: None,
    };
    let mut h: HashMap<String, CountryInfoS> = HashMap::new();
    h.insert("AFG".to_string(), c);
    println!("{:#?}", serde_json::to_string(&h));
    println!("test_country_load {}", COUNTRIES.len());
    assert!(COUNTRIES.len() > 0);
}


// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

include!("country-data.rs");

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_region_codes() {
        println!(">>> test_region_codes");
        let codes = region_codes();
        assert!(codes.len() > 0);
    }

    #[test]
    fn test_good_region_code() {
        match lookup_region(21) {
            None => panic!("was expecting a region"),
            Some(region) => assert_eq!(region.name, "Northern America"),
        }
    }

    #[test]
    fn test_bad_region_code() {
        match lookup_region(0) {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_country_codes() {
        let codes = country_codes();
        assert!(codes.len() > 0);
    }

    #[ignore]
    #[test]
    fn test_good_country_code() {
        match lookup_country("DEU") {
            None => panic!("was expecting a country"),
            Some(country) => {
                assert_eq!(country.short_code, "DE");
                assert_eq!(country.country_code, 276);
            },
        }
    }

    #[ignore]
    #[test]
    fn test_good_country_short_code() {
        match lookup_country("DE") {
            None => panic!("was expecting a country"),
            Some(country) => {
                assert_eq!(country.code, "DEU");
                assert_eq!(country.country_code, 276);
            },
        }
    }

    #[test]
    fn test_bad_country_code() {
        match lookup_country("XXX") {
            None => (),
            Some(_) => panic!("was expecting a None in response"),
        }
    }
}