/*!
Provides a layer above the `LocaleString` for different locale specifiers.

# The POSIX Locale

The "POSIX", or "C", locale is the minimal locale. It is a rather neutral
locale which has the same settings across all systems and compilers, and
therefore the exact results of a program using this locale are predictable.
This is the locale used by default on all C programs.

# Example

TBD
*/

use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use crate::string::{LocaleString, ParseError};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub enum Locale {
    POSIX,
    Path(PathBuf),
    String(LocaleString),
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
                Locale::POSIX => L_POSIX.to_string(),
                Locale::Path(s) => s.to_str().unwrap().to_string(),
                Locale::String(s) => s.to_string(),
            }
        )
    }
}

impl FromStr for Locale {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(ParseError::EmptyString);
        }
        match s {
            L_C  => Ok(Locale::POSIX),
            L_POSIX => Ok(Locale::POSIX),
            _ => {
                if s.starts_with("/") {
                    match PathBuf::from_str(s) {
                        Ok(p) => Ok(Locale::Path(p)),
                        Err(_) => Err(ParseError::InvalidPath),
                    }
                } else {
                    Ok(Locale::String(LocaleString::from_str(s)?))
                }
            },
        }
    }
}
