/*!

## Standards

* https://en.wikipedia.org/wiki/Locale_(computer_software)
* https://en.wikipedia.org/wiki/IETF_language_tag (https://tools.ietf.org/html/bcp47)
* https://www.w3.org/TR/ltli/
* https://en.wikipedia.org/wiki/ISO/IEC_15897 (https://www.iso.org/standard/50707.html, http://www.open-std.org/jtc1/sc22/wg20/docs/n610.pdf)

> On POSIX platforms such as Unix, Linux and others, locale identifiers are defined by ISO/IEC 15897, which is similar to the BCP 47 definition of language tags, but the locale variant modifier is defined differently, and the character set is included as a part of the identifier. It is defined in this format: [language[_territory][.codeset][@modifier]]. (For example, Australian English using the UTF-8 encoding is en_AU.UTF-8.)

<code>
language[_territory[.codeset]][@modifier]
</code>

* language = https://en.wikipedia.org/wiki/ISO_639-1
* country/territory = https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2 / https://en.wikipedia.org/wiki/UN_M49
* script/encoding = https://en.wikipedia.org/wiki/ISO_15924 (http://unicode.org/iso15924/iso15924-codes.html) / https://en.wikipedia.org/wiki/ISO/IEC_8859
* modifier = ?

See also:

* https://www.gnu.org/software/libc/manual/html_node/Locale-Names.html
* https://developer.apple.com/documentation/foundation/nslocale/1416263-localeidentifier
* https://developer.apple.com/documentation/foundation/nslocale
* https://docs.microsoft.com/en-us/cpp/c-runtime-library/locale-names-languages-and-country-region-strings?view=vs-2019
* https://docs.microsoft.com/en-us/windows/win32/intl/locale-names
*/
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use std::collections::HashMap;

use crate::codes::country;
use crate::codes::language;

pub struct LocaleString {
    strict: bool,
    language_code: String,
    territory: Option<String>,
    code_set: Option<String>,
    modifier: Option<String>,
}

const SEP_TERRITORY: char = '_';
const SEP_CODE_SET: char = '.';
const SEP_MODIFIER: char = '@';

const SEP_ALL: &'static str = "_.@";

/* see https://en.wikipedia.org/wiki/Character_encoding */
#[allow(non_camel_case_types)]
pub enum CodeSet {
    ANSEL,
    ArmSCII_7,
    ArmSCII_8,
    ArmSCII_8A,
    ASCII, // prefer US_ASCII
    US_ASCII,
    Big5, // replaces/encodes CCCII
    Big5_HKSCS,
    CNS_11643,
    CP_866,
    CP_936,
    CP_949,
    CP_1131,
    CP_1251,
    CP_1386,
    EBCDIC,
    EUC_CN,
    EUC_JP,
    EUC_KR,
    GB_2312,
    GBK, // 2312 extension
    GB_18030,
    ISCII,
    ISCII_BE,
    ISCII_BNG,
    ISCII_DE,
    ISCII_DEV,
    ISCII_GU,
    ISCII_GUJ,
    ISCII_KA,
    ISCII_KND,
    ISCII_MA,
    ISCII_MLM,
    ISCII_OR,
    ISCII_ORI,
    ISCII_PE,
    ISCII_GUR,
    ISCII_TA,
    ISCII_TML,
    ISCII_TE,
    ISCII_TIG,
    ISO_646, // ASCII
    ISO_6937, // ANSEL
    ISO_8859_1,
    ISO_8859_2,
    ISO_8859_3,
    ISO_8859_4,
    ISO_8859_5,
    ISO_8859_6,
    ISO_8859_7,
    ISO_8859_8,
    ISO_8859_9,
    ISO_8859_10,
    ISO_8859_11,
    ISO_8859_12,
    ISO_8859_13,
    ISO_8859_14,
    ISO_8859_15,
    ISO_8859_16,
    ISO_10585, // ArmSCII
    ISO_10646,
    KOI7,
    KOI8,
    KOI8_R,
    KOI8_RU,
    KOI8_T,
    KOI8_U,
    PASCII,
    PT154,
    Shift_JIS,
    SJIS,
    TIS_620,
    TSCII,
    UTF_7,
    UTF_8,
    UTF_16,
    UTF_32,
    VISCII,
    Other(String),
}

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
        assert_eq!(
            territory.len(),
            2,
            "territory codes are two character only"
        );
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

    pub fn with_code_set(&self, code_set: CodeSet) -> Self {
        LocaleString {
            strict: self.strict,
            language_code: self.language_code.clone(),
            territory: self.territory.clone(),
            code_set: Some(code_set.to_string()),
            modifier: self.modifier.clone(),
        }
    }

    pub fn with_code_set_string(&self, code_set: String) -> Self {
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
        where K: Display, V: Display {
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
        assert!(
            result.is_some(),
            "language code does not exist"
        );
    }

    fn test_known_territory(territory: &String) {
        let country_key = territory.clone();
        let result = &country::lookup_country(&country_key);
        assert!(
            result.is_some(),
            "territory code does not exist"
        );
    }
}

impl Display for LocaleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", [
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
        ].join(""))
    }
}

impl Display for CodeSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            CodeSet::ANSEL=> "ANSEL",
            CodeSet::ArmSCII_7 => "ARMSCII-7",
            CodeSet::ArmSCII_8=> "ARMSCII-8",
            CodeSet::ArmSCII_8A=> "ARMSCII-8A",
            CodeSet::ASCII=> "ASCII",
            CodeSet::US_ASCII=> "US-ASCII",
            CodeSet::Big5=> "Big5",
            CodeSet::Big5_HKSCS=> "Big5HKSCS",
            CodeSet::CNS_11643 => "CNS11643",
            CodeSet::CP_866=> "CP866",
            CodeSet::CP_936=> "CP936",
            CodeSet::CP_949=> "CP949",
            CodeSet::CP_1131=> "CP1131",
            CodeSet::CP_1251=> "CP1251",
            CodeSet::CP_1386=> "CP1386",
            CodeSet::EBCDIC=> "EBCDIC",
            CodeSet::EUC_CN=> "eucCN",
            CodeSet::EUC_JP=> "eucJP",
            CodeSet::EUC_KR=> "eucKR",
            CodeSet::GB_2312=> "GB2312",
            CodeSet::GBK=> "GBK", // 2312 extension
            CodeSet::GB_18030=> "GB18030",
            CodeSet::ISCII=> "ISCII",
            CodeSet::ISCII_BE=> "ISCII-BE",
            CodeSet::ISCII_BNG=> "ISCII-BNG",
            CodeSet::ISCII_DE=> "ISCII-DE",
            CodeSet::ISCII_DEV=> "ISCII-DEV",
            CodeSet::ISCII_GU => "ISCII-GU",
            CodeSet::ISCII_GUJ => "ISCII-GUJ",
            CodeSet::ISCII_KA=> "ISCII-KA",
            CodeSet::ISCII_KND=> "ISCII-KND",
            CodeSet::ISCII_MA=> "ISCII-MA",
            CodeSet::ISCII_MLM=> "ISCII-MLM",
            CodeSet::ISCII_OR=> "ISCII-OR",
            CodeSet::ISCII_ORI=> "ISCII-ORI",
            CodeSet::ISCII_PE=> "ISCII-PE",
            CodeSet::ISCII_GUR=> "ISCII-GUR",
            CodeSet::ISCII_TA=> "ISCII-TA",
            CodeSet::ISCII_TML=> "ISCII-TML",
            CodeSet::ISCII_TE=> "ISCII-TE",
            CodeSet::ISCII_TIG=> "ISCII-TIG",
            CodeSet::ISO_646 => "ISO646",
            CodeSet::ISO_6937 => "ISO6937",
            CodeSet::ISO_8859_1 => "ISO8859-1",
            CodeSet::ISO_8859_2 => "ISO8859-2",
            CodeSet::ISO_8859_3 => "ISO8859-3",
            CodeSet::ISO_8859_4 => "ISO8859-4",
            CodeSet::ISO_8859_5 => "ISO8859-5",
            CodeSet::ISO_8859_6 => "ISO8859-6",
            CodeSet::ISO_8859_7 => "ISO8859-7",
            CodeSet::ISO_8859_8 => "ISO8859-8",
            CodeSet::ISO_8859_9 => "ISO8859-9",
            CodeSet::ISO_8859_10 => "ISO8859-10",
            CodeSet::ISO_8859_11 => "ISO8859-11",
            CodeSet::ISO_8859_12 => "ISO8859-12",
            CodeSet::ISO_8859_13 => "ISO8859-13",
            CodeSet::ISO_8859_14 => "ISO8859-14",
            CodeSet::ISO_8859_15 => "ISO8859-15",
            CodeSet::ISO_8859_16 => "ISO8859-16",
            CodeSet::ISO_10585 => "ISO10585",
            CodeSet::ISO_10646 => "ISO10646",
            CodeSet::KOI7 => "KOI7",
            CodeSet::KOI8_R => "KOI8-R",
            CodeSet::KOI8 => "KOI8",
            CodeSet::KOI8_RU => "KOI8-RU",
            CodeSet::KOI8_T => "KOI8-T",
            CodeSet::KOI8_U => "KOI8-U",
            CodeSet::PASCII => "PASCII",
            CodeSet::PT154 => "PT154",
            CodeSet::Shift_JIS => "SJIS",
            CodeSet::SJIS => "SJIS",
            CodeSet::TIS_620 => "TIS-620",
            CodeSet::TSCII => "TSCII",
            CodeSet::UTF_7 => "UTF-7",
            CodeSet::UTF_8 => "UTF-8",
            CodeSet::UTF_16 => "UTF-16",
            CodeSet::UTF_32 => "UTF-32",
            CodeSet::VISCII => "VISCII",
            CodeSet::Other(s) => s,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    InvalidLanguageCode,
    InvalidCountryCode,
    InvalidCodeSet,
    InvalidModifier,
}

impl FromStr for LocaleString {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(ParseError::EmptyString)
        }
        let (mut locale, remaining) = match s.find(|c: char| SEP_ALL.contains(c)) {
            None => return Ok(LocaleString::new(s.to_string())),
            Some(i) => {
                ( LocaleString::new(s[..i].to_string()), &s[i..] )
            }
        };
        let mut remaining = remaining;
        while remaining.len() > 0 {
            let separator = remaining.chars().nth(0).unwrap();
            remaining = &remaining[1..];
            match remaining.find(|c: char| SEP_ALL.contains(c)) {
                None => return match separator {
                    SEP_TERRITORY => Err(ParseError::InvalidCountryCode),
                    SEP_CODE_SET => Err(ParseError::InvalidCodeSet),
                    SEP_MODIFIER => Err(ParseError::InvalidModifier),
                    _ => panic!("this shouldn't happen"),
                },
                Some(i) => {
                    let field = &remaining[..i];
                    remaining = &remaining[i..];
                    match separator {
                        SEP_TERRITORY => locale = locale.with_territory(field.to_string()),
                        SEP_CODE_SET => locale = locale.with_code_set_string(field.to_string()),
                        SEP_MODIFIER => locale = locale.with_modifier(field.to_string()),
                        _ => panic!("this shouldn't happen"),
                    }
                }
            }
        }
        Err(ParseError::EmptyString)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{LocaleString, CodeSet};

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
        LocaleString::new("en".to_string())
            .with_territory("USA".to_string());
    }

    #[test]
    #[should_panic(expected = "territory codes are upper case only")]
    fn test_bad_country_case() {
        LocaleString::new("en".to_string())
            .with_territory("us".to_string());
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
            locale.with_code_set(CodeSet::ISO_8859_13).get_code_set(),
            Some("ISO8859-13".to_string())
        );
    }

    #[test]
    fn test_with_code_set_string() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(
            locale.with_code_set_string("UTF-8".to_string()).get_code_set(),
            Some("UTF-8".to_string())
        );
    }

    #[test]
    fn test_with_modifier() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(
            locale.with_modifier("collation=pinyin;currency=CNY".to_string()).get_modifier(),
            Some("collation=pinyin;currency=CNY".to_string())
        );
    }

    #[test]
    fn test_with_modifiers() {
        let locale = LocaleString::new("en".to_string());
        let modifiers: HashMap<&str, &str> = [
            ("collation", "pinyin"),
            ("currency", "CNY")
            ]
            .iter()
            .cloned()
            .collect();
        assert!(
            locale.with_modifiers(modifiers).get_modifier().unwrap().contains("collation=pinyin")
        );
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
            .with_code_set(CodeSet::UTF_8)
            .with_modifier("collation=pinyin;currency=CNY".to_string());
        assert_eq!(locale.to_string(), "en_US.UTF-8@collation=pinyin;currency=CNY".to_string());
    }
}
