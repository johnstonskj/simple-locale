# Crate simple-locale

An idiomatic Rust crate for locale, and locale-like, operations.

This crate provides a higher-level interface to a number of locale-related
sources, in three areas:

1. Locale-related codes/identifiers and any standards-based information
   concerning them. For example, ISO-396 language identifiers, or ISO-3166
   country identifiers. These are under the module `simple_locale::codes`.
1. Locale settings, usually accessed via POSIX (see
   [ISO/IEC 15897](https://www.iso.org/standard/50707.html) operating system
   functions. These are under the module `simple_locale::settings`.
1. A `Locale` enumeration, and a `LocaleString` structure are provided
   that may be used to parse and construct locale identifiers in
   a standards-conformant manner.

## Example

```rust
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

## Pre-Build Process

The following describe two code generation steps that are executed outside
the normal build process as the output is stored in Git and versioned 
based on external factors.

### FFI Bindings

As mentioned above, this crate depends on FFI bindings to POSIX locale
functions, and there are O/S differences that make this a pain. The script
[`create-bindings.sh`](https://github.com/johnstonskj/simple-locale/blob/master/create-bindings.sh)
is used to generate these bindings (using cargo bindgen) in such a way that
different O/S bindings can be built effectively.

### JSON Data Files

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

