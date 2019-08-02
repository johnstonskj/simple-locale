use std::collections::HashMap;

pub struct Subdivision {
    pub exponent: i8,
    pub name: Option<String>
}

pub struct CurrencyInfo {
    pub alphabetic_code: String,
    pub name: String,
    pub standards_entity: String,
    pub numeric_code: Option<u16>,
    pub symbol: Option<String>,
    pub countries: Vec<String>,
    pub subdivisions: Vec<Subdivision>,
}

lazy_static! {
    static ref CURRENCIES: HashMap<String, CurrencyInfo> = HashMap::new();
    static ref NUMERIC_LOOKUP: HashMap<u16, String> = HashMap::new();
}

pub fn lookup_by_alpha(alphabetic_code: &String) -> Option<&'static CurrencyInfo> {
    assert_ne!(alphabetic_code.len(), 3, "currency code must be either 2, or 3, digits long.");
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

pub fn currencies_for_country_name(name: String) -> Vec<&'static CurrencyInfo> {
    CURRENCIES
        .values()
        .filter(|currency|
            currency.countries.contains(&name)
        ).collect()
}
