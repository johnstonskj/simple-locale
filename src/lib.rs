pub enum LocaleError {
    InvalidLocaleString,
    UnknownLocale,
    Unsupported,
}

pub type LocaleResult<T> = Result<T, LocaleError>;

mod ffi;

pub mod codes;

pub mod string;
pub use string::LocaleString;

pub mod settings;
pub use settings::LocaleScope;