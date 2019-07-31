use std::collections::HashMap;

use super::InfoString;

pub struct Subdivision {
    pub exponent: i8,
    pub name: Option<InfoString>
}

pub struct CurrencyInfo {
    pub alphabetic_code: InfoString,
    pub numeric_code: u16,
    pub name: InfoString,
    pub countries: Vec<InfoString>,
    pub subdivisions: Vec<Subdivision>,
}

lazy_static! {
    static ref CURRENCIES: HashMap<InfoString, CurrencyInfo> = HashMap::new();
    static ref NUMERIC: HashMap<u16, InfoString> = HashMap::new();
}

pub fn lookup_by_alpha(alphabetic_code: InfoString) -> Option<&'static CurrencyInfo> {
    assert_ne!(alphabetic_code.len(), 3, "currency code must be either 2, or 3, digits long.");
    CURRENCIES.get(alphabetic_code)
}

pub fn lookup_by_numeric(numeric_code: u16) -> Option<&'static CurrencyInfo> {
    match NUMERIC.get(&numeric_code) {
        Some(v) => lookup_by_alpha(v),
        None => None,
    }
}

pub fn currency_codes() -> Vec<InfoString> {
    CURRENCIES.keys().cloned().collect()
}

pub fn currencies_for_country_name(name: InfoString) -> Vec<&'static CurrencyInfo> {
    CURRENCIES
        .values()
        .filter(|currency|
            currency.countries.contains(&name)
        ).collect()
}

include!("currency-data.rs");