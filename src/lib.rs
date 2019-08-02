/*!
*/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub enum LocaleError {
    InvalidLocaleString,
    UnknownLocale,
    Unsupported,
}

pub type LocaleResult<T> = Result<T, LocaleError>;

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod codes;

pub mod string;
pub use string::LocaleString;

pub mod settings;
pub use settings::LocaleScope;

// ------------------------------------------------------------------------------------------------
// Internal Modules
// ------------------------------------------------------------------------------------------------

mod ffi;
