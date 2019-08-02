use super::LocaleScope;
use crate::string::LocaleString;
use crate::{LocaleError, LocaleResult};

#[derive(Debug, Clone)]
pub struct NumericSettings {
    pub decimal_sep: String,
    pub thousands_sep: String,
    pub grouping: Vec<usize>,
}

pub struct NumericScope {}

impl LocaleScope for NumericScope {
    fn get_locale() -> LocaleResult<LocaleString> {
        Ok(LocaleString::new("en".to_string()))
    }

    fn set_locale(_locale: LocaleString) -> LocaleResult<()> {
        Ok(())
    }

    fn get_settings<NumericSettings>() -> LocaleResult<NumericSettings> {
        Err(LocaleError::UnknownLocale)
    }
}

#[cfg(test)]
mod tests {
    use super::NumericScope;
    use crate::LocaleScope;

    #[test]
    fn test_get_locale() {
        if let Err(_) = NumericScope::get_locale() {
            panic!("expecting something better than that");
        }
    }
}
