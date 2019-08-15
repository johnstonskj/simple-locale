/*!
Fetch locale-specific message formatting settings.

This module is relatively simple as only a small number of
settings are defined by POSIX for the messages category.
*/

use crate::ffi::langinfo;
use crate::ffi::utils::*;
use crate::LocaleResult;

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
pub fn get_message_format() -> LocaleResult<MessageFormat> {
    Ok(MessageFormat {
        code_set: get_nl_string(langinfo::CODESET),
        yes_expression: get_nl_string(langinfo::YESEXPR),
        yes_string: get_nl_string(langinfo::YESSTR),
        no_expression: get_nl_string(langinfo::NOEXPR),
        no_string: get_nl_string(langinfo::NOSTR),
    })
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::settings::messages::get_message_format;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_something() {
        println!("{:#?}", get_message_format());
    }
}
