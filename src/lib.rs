pub enum LocaleError {
    InvalidLocaleString,
    UnknownLocale,
    Unsupported,
}

pub type LocaleResult<T> = Result<T, LocaleError>;

mod ffi;

mod scope;
pub use scope::LocaleScope;

pub mod currency;
pub use currency::CurrencyScope;

pub mod numeric;
pub use numeric::NumericScope;

pub mod string;
pub use string::LocaleString;

pub mod time;
pub use time::TimeScope;
