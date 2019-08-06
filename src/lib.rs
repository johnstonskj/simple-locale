/*!
An higher-level interface to all manner of locale-related information.

This crate provides a higher-level interface to a number of locale-related
sources, in three areas:

1. Locale-related codes/identifiers and any standards-based information
   concerning them. For example, ISO-396 language identifiers, or ISO-3166
   country identifiers. These are under the module
   [`simple_locale::codes`](codes/index.html).
1. Locale settings, usually accessed via POSIX (see
   [ISO/IEC 15897](https://www.iso.org/standard/50707.html) operating system
   functions. These are under the module
   [`simple_locale::settings`](settings/index.html).
1. A [`Locale`](locale/enum.Locale.html) enumeration, and a
   [`LocaleString`](string/struct.LocaleString.html) structure are provided
   that may be used to parse and construct locale identifiers in
   a standards-conformant manner.

This crate uses bindgen for the creation of operating system bindings to the
`langinfo`, `localcharset`, `locale`, and `xlocale` headers. Another
crate () does something similar, however...

## Example

The following example demonstrates some of the components of the crate, at
least some reasonable use cases.

1. Construct a _strict_ locale string where identifiers are checked against
   known standard codes where possible.
1. Lookup the ISO-3166 data for the country (in the
   [`CountryInfo`](codes/country/struct.CountryInfo.html) struct) identified
   by the ISO-3166, part 2, 3-character identifier.
1. The data fromn the last call contains one or more regions (in the
   [`RegionInfo`](/codes/country/struct.RegionInfo.html) struct), determine
   the countries name from the `country_code`.
1. Now we have the country name we can lookup the details of the currencies
   (in, the [`CurrencyInfo`](CurrencyInfo) struct).

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

## FFI Bindings

As mentioned above, this crate depends on FFI bindings to POSIX locale
functions, and there are O/S differences that make this a pain. The script
[`create-bindings.sh`](https://github.com/johnstonskj/simple-locale/blob/master/create-bindings.sh)
is used to generate these bindings (using cargo bindgen) in such a way that
different O/S bindings can be built effectively.

## JSON Data Files

The script [`create-data-modules`](https://github.com/johnstonskj/simple-locale/blob/master/create-data-modules.sh)
on the other hand is used to process files downloaded, or scraped, from
standards web sites to create data used by the library. This data is generated
as JSON files in the `src/codes/data` folder and read as a part of the
build for `codes` modules using the Rust `include!` macro.

Currently data is generated for the following standards:

* ISO 639 _Codes for the representation of names of languages_; Parts 1-4,
  2-character and 3-character codes supported.
* ISO 3166 _Codes for the representation of names of countries and their
  subdivisions_; Part 1, 2-character codes, only.
* ISO 4217 _Codes for the representation of currencies_; alphabetic and
  numeric codes supported.
* ISO 15924 _Codes for the representation of names of scripts_; alphabetic
  and numeric codes supported.

Each folder under `src-data` represents a single standard, which may
generate one or more data sets. Each directory will contain a Python
script, `generate.py` which is called by the top-level script to create
the JSON in the correct location. Each should also contain a README
that includes attribution for any data retrieved to make this possible.

*/

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate regex;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Common error type for functions in this crate.
pub enum LocaleError {
    /// The provided locale string was badly formatted
    InvalidLocaleString,
    /// The provided locale was unknown
    UnknownLocale,
    /// The operation you tried to perform was not supported.
    Unsupported,
}

/// Common result type for functions in this crate.
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
