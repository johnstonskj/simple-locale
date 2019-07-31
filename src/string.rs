use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

pub struct LocaleString {
    language_code: String,
    country_code: Option<String>,
    modifier: Option<String>,
}

impl LocaleString {
    pub fn new(language_code: String) -> Self {
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
        LocaleString {
            language_code,
            country_code: None,
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
        LocaleString {
            language_code,
            country_code: self.country_code.clone(),
            modifier: self.modifier.clone(),
        }
    }

    pub fn with_country(&self, country_code: String) -> Self {
        assert_eq!(
            country_code.len(),
            2,
            "country codes are two character only"
        );
        assert_eq!(
            country_code.chars().all(|c| c.is_uppercase()),
            true,
            "country codes are upper case only"
        );
        LocaleString {
            language_code: self.language_code.clone(),
            country_code: Some(country_code),
            modifier: self.modifier.clone(),
        }
    }

    pub fn with_modifier(&self, modifier: String) -> Self {
        LocaleString {
            language_code: self.language_code.clone(),
            country_code: self.country_code.clone(),
            modifier: Some(modifier),
        }
    }

    pub fn get_language_code(&self) -> String {
        self.language_code.clone()
    }

    pub fn get_country_code(&self) -> Option<String> {
        self.country_code.clone()
    }

    pub fn get_modifier(&self) -> Option<String> {
        self.modifier.clone()
    }

    pub fn to_lc_string(&self) -> String {
        [
            self.language_code.clone(),
            match &self.country_code {
                Some(v) => format!("_{}", v),
                None => "".to_string(),
            },
            match &self.modifier {
                Some(v) => format!(".{}", v),
                None => "".to_string(),
            },
        ]
        .join("")
    }

    pub fn to_w3_string(&self) -> String {
        [
            self.language_code.clone(),
            match &self.country_code {
                Some(v) => format!("-{}", v),
                None => "".to_string(),
            },
            match &self.modifier {
                Some(v) => format!("-{}", v),
                None => "".to_string(),
            },
        ]
        .join("")
    }
}

impl Display for LocaleString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_lc_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyString,
    InvalidLanguageCode,
    InvalidCountryCode,
}

impl FromStr for LocaleString {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Err(ParseError::EmptyString)
    }
}

#[cfg(test)]
mod tests {
    use crate::string::LocaleString;

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_constructor_length() {
        LocaleString::new("english".to_string());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_constructor_case() {
        LocaleString::new("EN".to_string());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_country_length() {
        LocaleString::new("en".to_string())
            .with_country("USA".to_string());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_bad_country_case() {
        LocaleString::new("en".to_string())
            .with_country("us".to_string());
    }

    #[test]
    fn test_constructor() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(locale.get_language_code(), "en".to_string());
        assert_eq!(locale.get_country_code(), None);
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
            locale.with_country("UK".to_string()).get_country_code(),
            Some("UK".to_string())
        );
    }

    #[test]
    fn test_with_modifier() {
        let locale = LocaleString::new("en".to_string());
        assert_eq!(
            locale.with_modifier("UTF-8".to_string()).get_modifier(),
            Some("UTF-8".to_string())
        );
    }

    #[test]
    fn test_lc_string() {
        let locale = LocaleString::new("en".to_string())
            .with_country("US".to_string())
            .with_modifier("UTF-8".to_string());
        assert_eq!(locale.to_lc_string(), "en_US.UTF-8".to_string());
    }
}
