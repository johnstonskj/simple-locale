use crate::settings::numeric::{NumericFormat, cstr_to_string, grouping_vector};
use crate::LocaleResult;
use crate::ffi::locale::localeconv;
use std::os::raw::c_char;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum SignLocation {
    UseParenthesis,
    BeforeString,
    AfterString,
    BeforeCurrencySymbol,
    AfterCurrencySymbol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignFormat {
    pub currency_symbol_precedes: bool,
    pub space_separated: bool,
    pub sign_location: SignLocation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyCellFormat {
    pub currency_symbol: String,
    pub decimal_precision: usize,
    pub positive_sign_format: SignFormat,
    pub negative_sign_format: SignFormat,
}

#[derive(Debug, Clone)]
pub struct CurrencyFormat {
    pub number_format: NumericFormat,
    pub local_format: Option<CurrencyCellFormat>,
    pub international_format: Option<CurrencyCellFormat>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_currency_format() -> LocaleResult<CurrencyFormat> {
    unsafe {
        let char_max = c_char::max_value();
        let lconv = localeconv();
        Ok(
            CurrencyFormat{
                number_format: NumericFormat {
                    positive_sign: cstr_to_string((*lconv).positive_sign),
                    negative_sign: cstr_to_string((*lconv).negative_sign),
                    decimal_separator: cstr_to_string((*lconv).mon_decimal_point),
                    thousands_separator: cstr_to_string((*lconv).mon_thousands_sep),
                    grouping: grouping_vector((*lconv).mon_grouping),
                },
                local_format: if (*lconv).frac_digits == char_max {
                    None
                } else {
                    Some(CurrencyCellFormat{
                        currency_symbol: cstr_to_string((*lconv).currency_symbol),
                        decimal_precision: (*lconv).frac_digits as usize,
                        positive_sign_format: SignFormat{
                            currency_symbol_precedes: ((*lconv).p_cs_precedes == 1),
                            space_separated: ((*lconv).p_sep_by_space == 1),
                            sign_location: match (*lconv).p_sign_posn {
                                0 => SignLocation::UseParenthesis,
                                1 => SignLocation::BeforeString,
                                2 => SignLocation::AfterString,
                                3 => SignLocation::BeforeCurrencySymbol,
                                4 => SignLocation::AfterCurrencySymbol,
                                _ => panic!("Bad sign location {}", (*lconv).p_sign_posn),
                            }
                        },
                        negative_sign_format: SignFormat{
                            currency_symbol_precedes: ((*lconv).n_cs_precedes == 1),
                            space_separated: ((*lconv).n_sep_by_space == 1),
                            sign_location: match (*lconv).n_sign_posn {
                                0 => SignLocation::UseParenthesis,
                                1 => SignLocation::BeforeString,
                                2 => SignLocation::AfterString,
                                3 => SignLocation::BeforeCurrencySymbol,
                                4 => SignLocation::AfterCurrencySymbol,
                                _ => panic!("Bad sign location {}", (*lconv).n_sign_posn),
                            }
                        }
                    })
                },
                international_format: if (*lconv).int_frac_digits  == char_max {
                    None
                } else {
                    Some(CurrencyCellFormat {
                        currency_symbol: cstr_to_string((*lconv).int_curr_symbol),
                        decimal_precision: (*lconv).int_frac_digits as usize,
                        positive_sign_format: SignFormat {
                            currency_symbol_precedes: ((*lconv).int_p_cs_precedes == 1),
                            space_separated: ((*lconv).int_p_sep_by_space == 1),
                            sign_location: match (*lconv).int_p_sign_posn {
                                0 => SignLocation::UseParenthesis,
                                1 => SignLocation::BeforeString,
                                2 => SignLocation::AfterString,
                                3 => SignLocation::BeforeCurrencySymbol,
                                4 => SignLocation::AfterCurrencySymbol,
                                _ => panic!("Bad sign location {}", (*lconv).int_p_sign_posn),
                            }
                        },
                        negative_sign_format: SignFormat {
                            currency_symbol_precedes: ((*lconv).int_n_cs_precedes == 1),
                            space_separated: ((*lconv).int_n_sep_by_space == 1),
                            sign_location: match (*lconv).int_n_sign_posn {
                                0 => SignLocation::UseParenthesis,
                                1 => SignLocation::BeforeString,
                                2 => SignLocation::AfterString,
                                3 => SignLocation::BeforeCurrencySymbol,
                                4 => SignLocation::AfterCurrencySymbol,
                                _ => panic!("Bad sign location {}", (*lconv).int_n_sign_posn),
                            }
                        }
                    })
                }
            }
        )
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use crate::{Locale, LocaleString};
    use crate::settings::locale::Category;
    use crate::settings::locale::api::*;
    use super::*;

    // --------------------------------------------------------------------------------------------
    # [test]
    fn test_currency_settings() {
        if get_locale(Category::Currency).unwrap() == Locale::POSIX {
            let format = get_currency_format().unwrap();
            println!("{:#?}", format);
            assert_eq!(format.number_format.decimal_separator, "");
            assert_eq!(format.local_format, None);
            assert_eq!(format.international_format, None);
        } else {
            warn!("didn't run test, too lazy to reset locale");
        }
    }

    // --------------------------------------------------------------------------------------------
    # [test]
    fn test_currency_settings_us() {
        let en_us = LocaleString::from_str("en_US.UTF-8").unwrap();

        if set_locale(&Locale::String(en_us), Category::Currency) {
            let format = get_currency_format().unwrap();
            println!("{:#?}", format);
            assert_eq!(format.number_format.decimal_separator, ".");
            assert_eq!(format.local_format.unwrap().currency_symbol, "$");
            assert_eq!(format.international_format.unwrap().currency_symbol, "USD ");
        }
    }
}