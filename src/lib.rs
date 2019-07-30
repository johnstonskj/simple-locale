pub enum LocaleError {
    InvalidLocaleString,
    UnknownLocale,
}

pub type LocaleResult<T> = Result<T, LocaleError>;

pub trait Locale {
    fn get_locale() -> LocaleResult<String>;

    fn set_locale(locale: String) -> LocaleResult<()>;
}

mod ffi;

pub mod currency;

pub mod numeric;

pub mod string;

pub mod time;
