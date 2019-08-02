/*!
The `LocaleString` type provides the a structure for locale identifier strings.

## Standards

> On POSIX platforms such as Unix, Linux and others, locale identifiers are defined by
> ISO/IEC 15897, which is similar to the BCP 47 definition of language tags, but the
> locale variant modifier is defined differently, and the character set is included as
> a part of the identifier.

Locale identifiers are defined in this format: `[language[_territory][.codeset][@modifier]]`.
For example, Australian English using the UTF-8 encoding is `en_AU.UTF-8`.

* `language` = [ISO 639-1](https://en.wikipedia.org/wiki/ISO_639-1) 2-character language
  codes.
* `territory` = [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) 2-character
  country codes.
* `codeset` = an undefined string value, `[a-zA-Z0-9_\-]+`.
  * For example, [IEC 8859](https://en.wikipedia.org/wiki/ISO/IEC_8859) parts 1 to 16 are
    usually specified as `ISO8859-1` and so on.
  * should be taken from the values in the IANA
    [character sets](https://www.iana.org/assignments/character-sets/character-sets.xhtml)
    list.
* `modifier` = a semi-colon separated list of _identifiers_, or _name '=' value_ pairs.
  * Sometimes this is used to indicate the language script in use, as such values from
    [ISO 15924](http://unicode.org/iso15924/iso15924-codes.html) should be used.

is the name registered by the MIT X Consortium that identifies the registration authority that owns the specific encoding. A modifier may be added to the registered name but is not required. The modifier is of the form @codeset modifier and identifies the coded character set as defined by that registration authority.

See also:

* https://www.gnu.org/software/libc/manual/html_node/Locale-Names.html
* https://developer.apple.com/documentation/foundation/nslocale/1416263-localeidentifier
* https://developer.apple.com/documentation/foundation/nslocale
* https://docs.microsoft.com/en-us/cpp/c-runtime-library/locale-names-languages-and-country-region-strings?view=vs-2019
* https://docs.microsoft.com/en-us/windows/win32/intl/locale-names
* https://en.wikipedia.org/wiki/Locale_(computer_software)
* https://en.wikipedia.org/wiki/IETF_language_tag (https://tools.ietf.org/html/bcp47)
* https://www.w3.org/TR/ltli/
* https://en.wikipedia.org/wiki/ISO/IEC_15897 (https://www.iso.org/standard/50707.html, http://www.open-std.org/jtc1/sc22/wg20/docs/n610.pdf)

*/
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use crate::codes::country;
use crate::codes::language;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub struct LocaleString {
    strict: bool,
    language_code: String,
    territory: Option<String>,
    code_set: Option<String>,
    modifier: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    InvalidLanguageCode,
    InvalidCountryCode,
    InvalidCodeSet,
    InvalidModifier,
    InvalidPath,
}

// ------------------------------------------------------------------------------------------------
// Implementations - LocaleString
// ------------------------------------------------------------------------------------------------

const SEP_TERRITORY: char = '_';
const SEP_CODE_SET: char = '.';
const SEP_MODIFIER: char = '@';

const SEP_ALL: &'static str = "_.@";

impl LocaleString {
    pub fn new(language_code: String) -> Self {
        LocaleString::common_new(language_code, false)
    }

    pub fn new_strict(language_code: String) -> Self {
        LocaleString::common_new(language_code, true)
    }

    fn common_new(language_code: String, strict: bool) -> Self {
        assert_eq!(
            language_code.len(),
            2,
            "language codes are two character only"
        );
        assert_eq!(
            language_code.chars().all(|c| c.is_lowercase()),
            true,
            "language codes are lower case only"
        );
        if strict {
            LocaleString::test_known_language(&language_code);
        }
        LocaleString {
            strict,
            language_code,
            territory: None,
            code_set: None,
            modifier: None,
        }
    }

    pub fn with_language(&self, language_code: String) -> Self {
        assert_eq!(
            language_code.len(),
            2,
            "language codes are two character only"
        );
        assert_eq!(
            language_code.chars().all(|c| c.is_lowercase()),
            true,
            "language codes are lower case only"
        );
        if self.strict {
            LocaleString::test_known_language(&language_code);
        }
        LocaleString {
            strict: false,
            language_code,
            territory: self.territory.clone(),
            code_set: self.code_set.clone(),
            modifier: self.modifier.clone(),
        }
    }

    pub fn with_territory(&self, territory: String) -> Self {
        assert_eq!(territory.len(), 2, "territory codes are two character only");
        assert_eq!(
            territory.chars().all(|c| c.is_uppercase()),
            true,
            "territory codes are upper case only"
        );
        if self.strict {
            LocaleString::test_known_territory(&territory);
        }
        LocaleString {
            strict: self.strict,
            language_code: self.language_code.clone(),
            territory: Some(territory),
            code_set: self.code_set.clone(),
            modifier: self.modifier.clone(),
        }
    }

    pub fn with_code_set(&self, code_set: String) -> Self {
        LocaleString {
            strict: self.strict,
            language_code: self.language_code.clone(),
            territory: self.territory.clone(),
            code_set: Some(code_set),
            modifier: self.modifier.clone(),
        }
    }

    pub fn with_modifier(&self, modifier: String) -> Self {
        LocaleString {
            strict: self.strict,
            language_code: self.language_code.clone(),
            territory: self.territory.clone(),
            code_set: self.code_set.clone(),
            modifier: Some(modifier),
        }
    }

    pub fn with_modifiers<K, V>(&self, modifiers: HashMap<K, V>) -> Self
    where
        K: Display,
        V: Display,
    {
        let modifier_strings: Vec<String> = modifiers
            .iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect();

        LocaleString {
            strict: self.strict,
            language_code: self.language_code.clone(),
            territory: self.territory.clone(),
            code_set: self.code_set.clone(),
            modifier: Some(modifier_strings.join(";")),
        }
    }

    pub fn get_language_code(&self) -> String {
        self.language_code.clone()
    }

    pub fn get_territory(&self) -> Option<String> {
        self.territory.clone()
    }

    pub fn get_code_set(&self) -> Option<String> {
        self.code_set.clone()
    }

    pub fn get_modifier(&self) -> Option<String> {
        self.modifier.clone()
    }

    fn test_known_language(language_code: &String) {
        let lang_key = language_code.clone();
        let result = &language::lookup(&lang_key);
        assert!(result.is_some(), "language code does not exist");
    }

    fn test_known_territory(territory: &String) {
        let country_key = territory.clone();
        let result = &country::lookup_country(&country_key);
        assert!(result.is_some(), "territory code does not exist");
    }
}

impl Display for LocaleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            [
                self.language_code.clone(),
                match &self.territory {
                    Some(v) => format!("{}{}", SEP_TERRITORY, v),
                    None => "".to_string(),
                },
                match &self.code_set {
                    Some(v) => format!("{}{}", SEP_CODE_SET, v),
                    None => "".to_string(),
                },
                match &self.modifier {
                    Some(v) => format!("{}{}", SEP_MODIFIER, v),
                    None => "".to_string(),
                },
            ]
            .join("")
        )
    }
}

impl FromStr for LocaleString {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(ParseError::EmptyString);
        }
        let (mut locale, remaining) = match s.find(|c: char| SEP_ALL.contains(c)) {
            None => return Ok(LocaleString::new(s.to_string())),
            Some(i) => (LocaleString::new(s[..i].to_string()), &s[i..]),
        };
        let mut remaining = remaining;
        while remaining.len() > 0 {
            let separator = remaining.chars().nth(0).unwrap();
            remaining = &remaining[1..];
            match remaining.find(|c: char| SEP_ALL.contains(c)) {
                None => {
                    return match separator {
                        SEP_TERRITORY => Err(ParseError::InvalidCountryCode),
                        SEP_CODE_SET => Err(ParseError::InvalidCodeSet),
                        SEP_MODIFIER => Err(ParseError::InvalidModifier),
                        _ => panic!("this shouldn't happen"),
                    }
                }
                Some(i) => {
                    let field = &remaining[..i];
                    remaining = &remaining[i..];
                    match separator {
                        SEP_TERRITORY => locale = locale.with_territory(field.to_string()),
                        SEP_CODE_SET => locale = locale.with_code_set(field.to_string()),
                        SEP_MODIFIER => locale = locale.with_modifier(field.to_string()),
                        _ => panic!("this shouldn't happen"),
                    }
                }
            }
        }
        Err(ParseError::EmptyString)
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::LocaleString;

    #[test]
    #[should_panic(expected = "language codes are two character only")]
    fn test_bad_constructor_length() {
        LocaleString::new("english".to_string());
    }

    #[test]
    #[should_panic(expected = "language codes are lower case only")]
    fn test_bad_constructor_case() {
        LocaleString::new("EN".to_string());
    }

    #[test]
    #[should_panic(expected = "territory codes are two character only")]
    fn test_bad_country_length() {
        LocaleString::new("en".to_string()).with_territory("USA".to_string());
    }

    #[test]
    #[should_panic(expected = "territory codes are upper case only")]
    fn test_bad_country_case() {
        LocaleString::new("en".to_string()).with_territory("us".to_string());
    }

    #[test]
    fn test_constructor() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(locale.get_language_code(), "en".to_string());
        assert_eq!(locale.get_territory(), None);
        assert_eq!(locale.get_modifier(), None);
    }

    #[test]
    fn test_with_language() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(
            locale.with_language("fr".to_string()).get_language_code(),
            "fr".to_string()
        );
    }

    #[test]
    fn test_with_country() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(
            locale.with_territory("UK".to_string()).get_territory(),
            Some("UK".to_string())
        );
    }

    #[test]
    fn test_with_code_set() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(
            locale.with_code_set("UTF-8".to_string()).get_code_set(),
            Some("UTF-8".to_string())
        );
    }

    #[test]
    fn test_with_modifier() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(
            locale
                .with_modifier("collation=pinyin;currency=CNY".to_string())
                .get_modifier(),
            Some("collation=pinyin;currency=CNY".to_string())
        );
    }

    #[test]
    fn test_with_modifiers() {
        let locale = LocaleString::new("en".to_string());
        let modifiers: HashMap<&str, &str> = [("collation", "pinyin"), ("currency", "CNY")]
            .iter()
            .cloned()
            .collect();
        assert!(locale
            .with_modifiers(modifiers)
            .get_modifier()
            .unwrap()
            .contains("collation=pinyin"));
        //        assert!(
        //            locale.with_modifiers(modifiers).get_modifier().unwrap().contains("currency=CNY")
        //        );
    }

    #[test]
    #[should_panic(expected = "language code does not exist")]
    fn test_strict_bad_language() {
        LocaleString::new_strict("xx".to_string());
    }

    #[test]
    #[should_panic(expected = "territory code does not exist")]
    fn test_strict_bad_territory() {
        let locale = LocaleString::new_strict("aa".to_string());
        locale.with_territory("XX".to_string());
    }

    #[test]
    fn test_strict_constructor() {
        let locale = LocaleString::new_strict("aa".to_string());
        assert_eq!(locale.get_language_code(), "aa".to_string());
    }

    #[test]
    fn test_lc_string() {
        let locale = LocaleString::new("en".to_string())
            .with_territory("US".to_string())
            .with_code_set("UTF-8".to_string())
            .with_modifier("collation=pinyin;currency=CNY".to_string());
        assert_eq!(
            locale.to_string(),
            "en_US.UTF-8@collation=pinyin;currency=CNY".to_string()
        );
    }
}
