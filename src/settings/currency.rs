use crate::settings::numeric::NumericFormat;
use crate::{LocaleError, LocaleResult};

pub enum SignLocation {
    UseParenthesis,
    BeforeString,
    AfterString,
    BeforeCurrencySymbol,
    AfterCurrencySymbol,
}

pub struct SignFormat {
    pub currency_symbol_precedes: bool,
    pub space_separated: bool,
    pub sign_location: SignLocation,
}
pub struct CurrencyFormat {
    pub number_format: NumericFormat,
    pub positive_sign: char,
    pub negative_sign: char,
    pub currency_symbol: String,
    pub fractional_digits: usize,
    pub positive_sign_format: SignFormat,
    pub negative_sign_format: SignFormat,
}

pub fn get_currency_format() -> LocaleResult<CurrencyFormat> {
    return Err(LocaleError::Unsupported);
}
