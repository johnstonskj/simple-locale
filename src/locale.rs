use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use crate::string::{LocaleString, ParseError};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub enum Locale {
    C,
    POSIX,
    Other(LocaleString),
}

// ------------------------------------------------------------------------------------------------
// Implementations - Locale
// ------------------------------------------------------------------------------------------------

const L_C: &'static str = "C";
const L_POSIX: &'static str = "POSIX";

impl Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Locale::C => L_C.to_string(),
                Locale::POSIX => L_POSIX.to_string(),
                Locale::Other(s) => s.to_string(),
            }
        )
    }
}

impl FromStr for Locale {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            L_C => Ok(Locale::C),
            L_POSIX => Ok(Locale::POSIX),
            _ => Ok(Locale::Other(LocaleString::from_str(s)?)),
        }
    }
}
