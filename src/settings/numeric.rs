/*!
Fetch locale-specific number formatting settings.

This module provides basic formatting rules for most rational
numbers; floating point numbers in scientific notation, fractional
numbers, and imaginary numbers are not covered.
*/

use crate::ffi::locale::localeconv;
use crate::ffi::utils::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Numeric formatting settings.
#[derive(Debug, Clone)]
pub struct NumericFormat {
    /// The symbol to use for positive values.
    pub positive_sign: String,
    /// The symbol to use for negative values.
    pub negative_sign: String,
    /// The radix character.
    pub decimal_separator: String, // TODO: is radix_character better?
    /// The separator between groups.
    pub thousands_separator: String, // TODO: is group_separator better?
    /// The grouping rules.
    pub grouping: Vec<usize>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Fetch the numeric formatting settings for the current locale.
pub fn get_numeric_format() -> NumericFormat {
    unsafe {
        let lconv = localeconv();
        NumericFormat {
            positive_sign: cstr_to_string((*lconv).positive_sign),
            negative_sign: cstr_to_string((*lconv).negative_sign),
            decimal_separator: cstr_to_string((*lconv).decimal_point),
            thousands_separator: cstr_to_string((*lconv).thousands_sep),
            grouping: grouping_vector((*lconv).grouping),
        }
    }
}

#[cfg(experimental)]
pub mod fmt {
    use std::fmt::Display;

    pub fn format<T>(value: T) -> String
        where T: Display {
        let initial = value.to_string();
        if initial.chars().all(|c| c.is_digit(10) || c == '.') {
            initial
        } else {
            panic!("doesn't look like a nunber");
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::locale::api::*;
    use crate::settings::locale::Category;
    use crate::Locale;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_numeric_settings() {
        if get_locale(Category::Currency).unwrap() == Locale::POSIX {
            let format = get_numeric_format();
            println!("{:#?}", format);
            assert_eq!(format.decimal_separator, ".");
        } else {
            warn!("didn't run test, too lazy to reset locale");
        }
    }
}
