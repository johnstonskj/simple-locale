use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct Subdivision {
    pub exponent: i8,
    pub name: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyInfo {
    pub alphabetic_code: String,
    pub name: String,
    pub numeric_code: Option<u16>,
    pub symbol: Option<String>,
    pub standards_entities: Vec<String>,
    pub subdivisions: Vec<Subdivision>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref CURRENCIES: HashMap<String, CurrencyInfo> = currencies_from_json();
    static ref NUMERIC_LOOKUP: HashMap<u16, String> = load_currency_lookup();
}

pub fn lookup_by_alpha(alphabetic_code: &String) -> Option<&'static CurrencyInfo> {
    assert_eq!(alphabetic_code.len(), 3, "currency code must be 3 characters long");
    CURRENCIES.get(alphabetic_code)
}

pub fn lookup_by_numeric(numeric_code: &u16) -> Option<&'static CurrencyInfo> {
    match NUMERIC_LOOKUP.get(&numeric_code) {
        Some(v) => lookup_by_alpha(v),
        None => None,
    }
}

pub fn currency_alpha_codes() -> Vec<String> {
    CURRENCIES.keys().cloned().collect()
}

pub fn currency_numeric_codes() -> Vec<u16> {
    NUMERIC_LOOKUP.keys().cloned().collect()
}

//pub fn currencies_for_country_name(name: String) -> Vec<&'static CurrencyInfo> {
//    CURRENCIES
//        .values()
//        .filter(|currency|
//            currency.countries.contains(&name)
//        ).collect()
//}

// ------------------------------------------------------------------------------------------------
// Generated Data
// ------------------------------------------------------------------------------------------------

fn currencies_from_json() -> HashMap<String, CurrencyInfo> {
    info!("currencies_from_json - loading JSON");
    let raw_data = include_bytes!("data/currencies.json");
    let currency_map: HashMap<String, CurrencyInfo> = serde_json::from_slice(raw_data).unwrap();
    info!("currencies_from_json - loaded {} currencies", currency_map.len());
    currency_map
}

fn load_currency_lookup() -> HashMap<u16, String> {
    info!("load_currency_lookup - create from CURRENCIES");
    let mut lookup_map: HashMap<u16, String> = HashMap::new();
    for currency in CURRENCIES.values() {
        if let Some(numeric) = &currency.numeric_code {
            lookup_map.insert(*numeric, currency.alphabetic_code.to_string());
        }
    }
    info!("load_currency_lookup - mapped {} countries", lookup_map.len());
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
    fn test_currency_loading() {
        match lookup_by_alpha(&"GBP".to_string()) {
            None => println!("lookup_by_alpha NO 'GBP'"),
            Some(c) => println!("lookup_by_alpha {:#?}", to_string_pretty(c)),
        }
    }
}