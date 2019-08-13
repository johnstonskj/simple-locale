use crate::LocaleResult;
use crate::ffi::utils::*;
use crate::ffi::langinfo;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct MessageFormat {
    pub code_set: Option<String>,
    pub yes_expression: Option<String>,
    pub yes_string: Option<String>,
    pub no_expression: Option<String>,
    pub no_string: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_message_format() -> LocaleResult<MessageFormat> {
    Ok(MessageFormat{
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
    # [test]
    fn test_something() {
        println!("{:#?}", get_message_format());
    }
}
