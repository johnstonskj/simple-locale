/*!
Parent to a set of operating system, locale functions.

Typically we treat each of the categories defined by POSIX in `locale.h` as
modules. The categories are show in the table below.

| POSIX Category | Module     | Function(s)                                  |
|================|============|==============================================|
| `LC_MESSAGES`  | `messages` | `get_message_format`                         |
| `LC_MONETARY`  | `currency` | `get_currency_format`                        |
| `LC_NUMERIC`   | `numeric`  | `get_numeric_format`                         |
|  `LC_TIME`     | `time`     | `get_date_time_format`, `get_calendar_names` |

The POSIX categories `LC_COLLATE`, and `LC_CTYPE` are not mapped to modules as
there are no calls to retrieve specific information regarding these. Additionally,
th module `locale` has the necessary functions to get and set the locale either
for an individual category or for all. This modules provides implementations
that use the C API directly as well as an implementation that uses environment
variables.

## Relationship to the POSIX API

The POSIX locale API is spread across a number of functions including
`locale_charset`, `localeconv`, `nl_langinfo`, and `setlocale`. Also, the
different categories of data is mixed up in common structures. The intent
of this crate is to invert this abstraction, to group together data by common
category regardless of the underlying API being used.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod locale;

pub mod currency;

pub mod messages;

pub mod numeric;

pub mod time;
