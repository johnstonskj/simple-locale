use crate::LocaleResult;
use crate::ffi::locale::localeconv;
use std::ffi::CStr;
use std::os::raw::c_char;

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

pub fn get_numeric_format() -> LocaleResult<NumericFormat> {
    unsafe {
        let lconv = localeconv();
        Ok(NumericFormat {
            positive_sign: cstr_to_string((*lconv).positive_sign),
            negative_sign: cstr_to_string((*lconv).negative_sign),
            decimal_separator: cstr_to_string((*lconv).decimal_point),
            thousands_separator: cstr_to_string((*lconv).thousands_sep),
            grouping: grouping_vector((*lconv).grouping),
        })
    }
}

pub(crate) unsafe fn cstr_to_string(c_str: *mut c_char) -> String {
    CStr::from_ptr(c_str).to_string_lossy().into_owned()
}

pub(crate) unsafe fn grouping_vector(grouping: *mut c_char) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    let mut c_ptr = grouping;
    loop {
        let a_char = *c_ptr;
        result.push(a_char as usize);
        if a_char == 0 || a_char == c_char::max_value() {
            break;
        }
        c_ptr = c_ptr.add(1);
    }
    result
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
            let format = get_numeric_format().unwrap();
            println!("{:#?}", format);
            assert_eq!(format.decimal_separator, ".");
        } else {
            warn!("didn't run test, too lazy to reset locale");
        }
    }
}