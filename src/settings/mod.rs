/*!
Parent to a set of operating system, locale functions.

The child modules of this one represent common areas, or _scope_ such as currency,
numbers, time, etc. that have settings available via the POSIX API. Each
module will have an implementation of the `LocaleScope` trait that allows a
client to get or set the locale associated with the scope as well as to get any
settings associated with the scope.

The structure that implements `LocaleScope` within each child module is re-exported
from this module.

## Relationship to the POSIX API

The POSIX locale API is spread across a number of functions including `locale_charset`,
`localeconv`, `nl_langinfo`, and `setlocale`. Also, the different scope of data is mixed
up in common structures. The intent of this crate is to invert this abstraction, to group
together data by common scope regardless of the underlying API being used.
*/

use crate::locale::Locale;
use crate::LocaleResult;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// This trait is implemented by each scope
///
pub trait LocaleScope {
    fn get_locale() -> LocaleResult<Locale>;

    fn set_locale(locale: Locale) -> LocaleResult<()>;

    fn get_settings<T>() -> LocaleResult<T>;
}

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod currency;
pub use currency::CurrencyScope;

pub mod numeric;
pub use numeric::NumericScope;

pub mod time;
pub use time::TimeScope;
