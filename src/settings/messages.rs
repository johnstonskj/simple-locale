/*!
Fetch locale-specific message formatting settings.

This module is relatively simple as only a small number of
settings are defined by POSIX for the messages category.
*/

use crate::ffi::langinfo;
use crate::ffi::utils::*;
use crate::{Locale, LocaleResult};

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

pub fn get_message_format_for_locale(
    locale: Locale,
    inherit_current: bool,
) -> LocaleResult<MessageFormat> {
    get_format_for_locale(locale, &get_message_format, inherit_current)
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
            assert_eq!(format.yes_expression, Some("^[yY]".to_string()));
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
            let format = get_message_format_for_locale(Locale::from_str("fr_FR").unwrap(), false);
            println!("{:#?}", format);
            let format = format.unwrap();
            assert_eq!(format.yes_expression, Some("^[oOyY].*".to_string()));
            assert_eq!(format.no_string, Some("no".to_string()));
        } else {
            panic!("expecting POSIX locale");
        }
    }
}
