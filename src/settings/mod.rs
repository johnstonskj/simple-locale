/*!
Parent to a set of operating system, locale functions.


## Relationship to the POSIX API

The POSIX locale API is spread across a number of functions including `locale_charset`,
`localeconv`, `nl_langinfo`, and `setlocale`. Also, the different scope of data is mixed
up in common structures. The intent of this crate is to invert this abstraction, to group
together data by common scope regardless of the underlying API being used.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod locale;

pub mod currency;

pub mod messages;

pub mod numeric;

pub mod time;
