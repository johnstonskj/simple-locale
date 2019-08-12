use crate::{LocaleError, LocaleResult};

#[derive(Debug, Clone)]
pub struct NumericFormat {
    pub decimal_separator: String,
    pub thousands_separator: String,
    pub grouping: Vec<usize>,
}

pub fn get_numeric_format() -> LocaleResult<NumericFormat> {
    return Err(LocaleError::Unsupported);
}
