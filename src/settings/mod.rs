use crate::string::LocaleString;
use crate::LocaleResult;

pub trait LocaleScope {
    fn get_locale() -> LocaleResult<LocaleString>;

    fn set_locale(locale: LocaleString) -> LocaleResult<()>;

    fn get_settings<T>() -> LocaleResult<T>;
}

pub mod currency;
pub use currency::CurrencyScope;

pub mod numeric;
pub use numeric::NumericScope;

pub mod time;
pub use time::TimeScope;
