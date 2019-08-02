/*!
An higher-level interface to all manner of locale-related information.

This crate provides a higher-level interface to a number of locale-related
sources. It does wrap the POSIX (see [ISO/IEC 15897](https://www.iso.org/standard/50707.html))
operating system functions,

## Example

```
use simple_locale::LocaleString;
use simple_locale::codes::{country, currency};

let locale = LocaleString::new_strict("en".to_string())
    .with_territory("US".to_string())
    .with_code_set("UTF-8".to_string())
    .with_modifier("collation=pinyin;currency=CNY".to_string());
println!("{}", locale);

let mexico = country::lookup_country("MEX").unwrap();
println!("{:?}", mexico);

let mexico_region = country::lookup_region(mexico.country_code).unwrap();
println!("{:?}", mexico_region);

let currencies = currency::currencies_for_country_name(mexico_region.name.as_str());
println!("{:?}", currencies);
```
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
