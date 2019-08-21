/*!
Parent to a set of operating system, locale functions.

Typically we treat each of the categories defined by POSIX in `locale.h` as
modules. The categories are show in the table below.

| POSIX Category | Module     | Function(s)                                  |
|----------------|------------|----------------------------------------------|
| `LC_COLLATE`   | N/A        | |
| `LC_CTYPE`     | N/A        | |
| `LC_MESSAGES`  | [`messages`](messages/index.html) | `get_message_format`, `get_message_format_for_locale` |
| `LC_MONETARY`  | [`currency`](currency/index.html) | `get_currency_format`, `get_currency_format_for_locale` |
| `LC_NUMERIC`   | [`numeric`](numeric/index.html)  | `get_numeric_format`, `get_numeric_format_for_locale` |
| `LC_TIME`      | [`time`](time/index.html)     | `get_date_time_format`, `get_date_time_format_for_locale`, `get_calendar_names`, `get_calendar_names_for_locale` |

> Note: the POSIX categories `LC_COLLATE`, and `LC_CTYPE` are not mapped to
> modules as there are no calls to retrieve specific information regarding these.

For each module there is _at least_ a matching pair of functions, one which takes
zero parameters and returns the current locale settings, and one which takes
two parameters and allows the retrieval of settings from another locale. The first
of these parameters is a [`Locale`](../locale/enum.Locale.html) enum that denotes
the locale to query, and the second parameter is a boolean `inherit_current` that
determines how the specified locale should be interpreted.

Additionally, the module [`locale`](locale/index.html) has the necessary functions
to get and set the locale either for an individual category or for all. This modules
provides implementations that use the C API directly as well as an implementation
that uses standard environment variables.

## Relationship to the POSIX API

The POSIX locale API is spread across a number of functions including
[`localeconv`](https://man.openbsd.org/localeconv.3),
[`nl_langinfo`](https://man.openbsd.org/nl_langinfo.3), and
[`setlocale`](https://man.openbsd.org/setlocale.3). Also, the
different categories of data is mixed up in common structures. The intent
of this crate is to invert this abstraction, to group together data by common
category regardless of the underlying API being used.

Those functions in the table above that end in `_for_locale` use the `xlocale`
extended API, specifically
[`newlocale`](https://man.openbsd.org/newlocale.3),
[`uselocale`](https://man.openbsd.org/uselocale.3), and
[`freelocale`](https://man.openbsd.org/freelocale.3) to obtain the settings
for a locale other than the current.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod locale;

pub mod currency;

pub mod messages;

pub mod numeric;

pub mod time;
