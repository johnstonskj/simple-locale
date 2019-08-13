use crate::ffi::locale::localeconv;
use crate::ffi::utils::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct NumericFormat {
    pub positive_sign: String,
    pub negative_sign: String,
    pub decimal_separator: String,
    pub thousands_separator: String,
    pub grouping: Vec<usize>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

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

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::Locale;
    use crate::settings::locale::Category;
    use crate::settings::locale::api::*;
    use super::*;

    // --------------------------------------------------------------------------------------------
    # [test]
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