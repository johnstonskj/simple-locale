/*!
Provides a layer above the `LocaleString` for different locale specifiers.

The `Locale` enum represents three forms of locale specification supported
by the POSIX C API. These are:

1. The identifier "POSIX", or "C" is known as the _minimal locale_. It is
   a rather neutral locale which has the same settings across all systems and
   compilers, and therefore the exact results of a program using this locale
   are predictable. This is the locale used by default on all C programs.
2. A path, starting with the '/' character and which resolves to a directory
   containing the POSIX definition of a locale.
3. A locale string, represented in this crate as a `LocaleString` structure
   that effectively represents a language with cultural qualification.

The `Locale::from_str` method can be used to parse any of these kinds into
the separate enumeration values.

## Examples

```
use simple_locale::locale::Locale;
use std::str::FromStr;

match Locale::from_str("C") {
    Ok(Locale::POSIX) => (),
    _ => panic!("could not parse first locale string")
}

let locale = Locale::from_str("en_US.UTF-8@Latn");
if let Ok(Locale::String(locale_str)) = locale {
    assert_eq!(locale_str.get_language_code(), "en".to_string());
    assert_eq!(locale_str.get_territory().unwrap(), "US".to_string());
    assert_eq!(locale_str.get_code_set().unwrap(), "UTF-8".to_string());
    assert_eq!(locale_str.get_modifier().unwrap(), "Latn".to_string());
} else {
    panic!("could not parse second locale string")
}
```

The following example is a more complete use case, the need to parse
the commonly used `LC_ALL` environment variable to determine it's type
and potential components.

```
use simple_locale::locale::Locale;
use std::str::FromStr;
use std::env;

if let Ok(lc_str) = env::var("LC_ALL") {
    match Locale::from_str(&lc_str) {
        Ok(Locale::POSIX) =>
            println!("POSIX minimal locale"),
        Ok(Locale::Path(p)) =>
            println!("Path locale"),
        Ok(Locale::String(s)) =>
            println!("String locale"),
        _ => panic!("Parse Error"),
    }
}

```
*/

use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;

use crate::string::{LocaleString, ParseError};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// This enumeration represents the three types of Locale specifiers
/// commonly used by operating systems.
#[derive(Debug, PartialEq)]
pub enum Locale {
    /// The minimal locale specified by POSIX. Can be spoecified with
    /// the string "POSIX" or simply "C".
    POSIX,
    /// A path to a locale specification, this library does not vslidste
    /// whether the path exists, simply that it is a valid `PathBuf`..
    Path(PathBuf),
    /// A locale string, parsed into a structured `LocaleString` form.
    String(LocaleString),
}

// ------------------------------------------------------------------------------------------------
// Implementations - Locale
// ------------------------------------------------------------------------------------------------

const L_C: &'static str = "C";
const L_POSIX: &'static str = "POSIX";
const L_PATH_SEP: &'static str = "/";

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
            L_C => Ok(Locale::POSIX),
            L_POSIX => Ok(Locale::POSIX),
            _ => {
                if s.starts_with(L_PATH_SEP) {
                    match PathBuf::from_str(s) {
                        Ok(p) => Ok(Locale::Path(p)),
                        Err(_) => Err(ParseError::InvalidPath),
                    }
                } else {
                    Ok(Locale::String(LocaleString::from_str(s)?))
                }
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_posix_to_string() {
        assert_eq!(Locale::POSIX.to_string(), "POSIX");
    }

    #[test]
    fn test_path_to_string() {
        let _path = PathBuf::from_str("/usr/share/locale/en_US");
        //        assert_eq!(path.to_string(), "/usr/share/locale/en_US");
    }

    #[test]
    fn test_string_to_string() {
        let locale = LocaleString::new("en".to_string())
            .with_territory("US".to_string())
            .with_code_set("UTF-8".to_string());
        assert_eq!(locale.to_string(), "en_US.UTF-8");
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_posix_from_string() {
        match Locale::from_str("POSIX") {
            Ok(Locale::POSIX) => (),
            _ => panic!("expecting Locale::POSIX"),
        }
        match Locale::from_str("C") {
            Ok(Locale::POSIX) => (),
            _ => panic!("expecting Locale::POSIX (C)"),
        }
    }

    #[test]
    fn test_path_from_string() {
        match Locale::from_str("/usr/share/locale/en_US") {
            Ok(Locale::Path(p)) => assert_eq!(p.to_str(), Some("/usr/share/locale/en_US")),
            _ => panic!("expecting Locale::Path"),
        }
    }

    #[test]
    fn test_string_from_string() {
        println!("{:#?}", Locale::from_str("en_US.UTF-8"));
        match Locale::from_str("en_US.UTF-8") {
            Ok(Locale::String(ls)) => {
                assert_eq!(ls.get_language_code(), "en");
                assert_eq!(ls.get_territory(), Some("US".to_string()));
                assert_eq!(ls.get_code_set(), Some("UTF-8".to_string()));
            }
            _ => panic!("expecting Locale::String"),
        }
    }
}
