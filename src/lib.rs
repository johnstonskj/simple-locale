/*!
An higher-level interface to all manner of locale-related information.

This crate provides a higher-level interface to a number of locale-related
sources. It does wrap the POSIX (see [ISO/IEC 15897](https://www.iso.org/standard/50707.html))
operating system functions,
*/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Common errors across the library.
pub enum LocaleError {
    /// The provided locale string was badly formatted
    InvalidLocaleString,
    /// The provided locale was unknown
    UnknownLocale,
    /// The operation you tried to perform was not supported.
    Unsupported,
}

pub type LocaleResult<T> = Result<T, LocaleError>;

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod codes;

pub mod string;
pub use string::LocaleString;

pub mod locale;
pub use locale::Locale;

pub mod settings;
pub use settings::LocaleScope;

// ------------------------------------------------------------------------------------------------
// Internal Modules
// ------------------------------------------------------------------------------------------------

mod ffi;
