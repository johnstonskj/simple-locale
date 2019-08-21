/*!
Fetch locale-specific message formatting settings.

This module is relatively simple as only a small number of
settings are defined by POSIX for the messages category.
*/

use crate::ffi::langinfo;
use crate::ffi::utils::*;
use crate::ffi::xlocale::{_xlocale, freelocale, newlocale, uselocale};
use crate::settings::locale::Category;
use crate::{Locale, LocaleError, LocaleResult, LocaleString};
use std::ptr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Settings related to message display.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageFormat {
    /// The code set to use when displaying messages in the current locale.
    /// Note that this does not return standard code set identifiers and so
    /// this value cannot be used with the `codes::codesets` module.
    pub code_set: Option<String>,
    /// An expression to validate 'yes' values.
    pub yes_expression: Option<String>,
    /// A default string for 'yes'.
    pub yes_string: Option<String>,
    /// An expression to validate 'no' values.
    pub no_expression: Option<String>,
    /// A default string for 'no'.
    pub no_string: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Fetch the message formatting settings for the current locale.
pub fn get_message_format() -> MessageFormat {
    MessageFormat {
        code_set: get_nl_string(langinfo::CODESET),
        yes_expression: get_nl_string(langinfo::YESEXPR),
        yes_string: get_nl_string(langinfo::YESSTR),
        no_expression: get_nl_string(langinfo::NOEXPR),
        no_string: get_nl_string(langinfo::NOSTR),
    }
}

pub fn get_message_format_for_locale(locale: Locale) -> LocaleResult<MessageFormat> {
    let os_loc = unsafe {
        let null_loc: *mut _xlocale = ptr::null_mut();
        let os_loc = newlocale(
            Category::Message.to_os_code() as i32,
            locale.to_string().as_ptr() as *const i8,
            null_loc,
        );
        if os_loc == null_loc {
            return Err(LocaleError::OSError);
        }
        uselocale(os_loc)
    };
    let format = get_message_format();
    unsafe {
        freelocale(os_loc);
    }
    Ok(format)
}

pub fn get_message_format_for_partial_locale(locale: LocaleString) -> LocaleResult<MessageFormat> {
    let os_loc = unsafe {
        let null_loc: *mut _xlocale = ptr::null_mut();
        let curr_loc = uselocale(null_loc);
        let os_loc = newlocale(
            Category::Message.to_os_code() as i32,
            locale.to_string().as_ptr() as *const i8,
            curr_loc,
        );
        if os_loc == null_loc {
            return Err(LocaleError::OSError);
        }
        uselocale(os_loc)
    };
    let format = get_message_format();
    unsafe {
        freelocale(os_loc);
    }
    Ok(format)
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::settings::locale::api::get_locale;
    use crate::settings::locale::Category;
    use crate::settings::messages::{get_message_format, get_message_format_for_locale};
    use crate::Locale;
    use std::str::FromStr;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_message_format() {
        let locale = get_locale(Category::Time).unwrap();
        if locale == Locale::POSIX {
            let format = get_message_format();
            println!("{:#?}", format);
            assert_eq!(format.yes_string, Some("yes".to_string()));
            assert_eq!(format.no_string, Some("no".to_string()));
        } else {
            panic!("expecting POSIX locale");
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_message_format_for_locale() {
        let locale = get_locale(Category::Time).unwrap();
        if locale == Locale::POSIX {
            let format = get_message_format_for_locale(Locale::from_str("fr_FR").unwrap());
            println!("{:#?}", format);
            let format = format.unwrap();
            assert_eq!(format.yes_string, Some("yes".to_string()));
            assert_eq!(format.no_string, Some("no".to_string()));
        } else {
            panic!("expecting POSIX locale");
        }
    }
}
