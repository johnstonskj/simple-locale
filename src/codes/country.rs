use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Region {
    pub code: u16,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CountryInfo {
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
    static ref REGIONS: HashMap<u16, Region> = load_regions_from_json();
    static ref COUNTRIES: HashMap<String, CountryInfo> = load_countries_from_json();
    static ref LOOKUP: HashMap<String, String> = load_country_lookup();
}

pub fn lookup_region(code: u16) -> Option<&'static Region> {
    info!("lookup_region: {}", code);
    match REGIONS.get(&code) {
        Some(v) => Some(v),
        None => None,
    }
}

pub fn lookup_country(code: &str) -> Option<&'static CountryInfo> {
    debug!("lookup_country: {}", code);
    assert!(code.len() == 2 || code.len() == 3, "country code must be either 2, or 3, characters long.");
    match code.len() {
        3 => {
            debug!("lookup_country: 3-character code");
            match COUNTRIES.get(code) {
                Some(v) => Some(v),
                None => None,
            }
        },
        2 => {
            debug!("lookup_country: 2-character code");
            match LOOKUP.get(code) {
                Some(v) =>
                    lookup_country(v)
                ,
                None => None,
            }
        },
        _ => None,
    }
}

pub fn region_codes() -> Vec<u16> {
    REGIONS.keys().cloned().collect()
}

pub fn country_codes() -> Vec<String> {
    COUNTRIES.keys().cloned().collect()
}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn load_regions_from_json() -> HashMap<u16, Region> {
    info!("load_regions_from_json - loading JSON");
    let raw_data = include_bytes!("data/regions.json");
    let raw_map: HashMap<String, String> = serde_json::from_slice(raw_data).unwrap();
    raw_map
        .iter()
        .map(|(code, name)|
            (
                code.parse::<u16>().unwrap(),
                Region {
                    code: code.parse::<u16>().unwrap(),
                    name: name.to_string()
                }))
        .collect()
}

fn load_countries_from_json() -> HashMap<String, CountryInfo> {
    info!("load_countries_from_json - loading JSON");
    let raw_data = include_bytes!("data/countries.json");
    let country_map: HashMap<String, CountryInfo> = serde_json::from_slice(raw_data).unwrap();
    info!("load_countries_from_json - loaded {} countries", country_map.len());
    country_map
}

fn load_country_lookup() -> HashMap<String, String> {
    info!("load_country_lookup - create from COUNTRIES");
    let mut lookup_map: HashMap<String, String> = HashMap::new();
    for country in COUNTRIES.values() {
        lookup_map.insert(country.short_code.to_string(), country.code.to_string());
    }
    info!("load_country_lookup - mapped {} countries", lookup_map.len());
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
    fn test_region_codes() {
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