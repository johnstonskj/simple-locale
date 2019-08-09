/*!
Parent to a set of standard code/identifier lookup modules.

These modules are effectively registries of standard code/identifiers and
any metadate published as a part of the associated standard(s). For example,
_Codes for the representation of currencies_, or ISO 4217 is based on a
spreadsheet published directly by ISO itself with some additional fields added
from other publicly accessible sources.

Modules typically implement a `lookup()` function (although where some standards
have both alphabetic and numeric identifiers there are `lookup_by_alpha()` and
`lookup_by_numeric()` instead) that returns an `Option`. Most will also include
a function `all_codes()` (or `all_alpha_codes()` and `all_numeric_codes()`) to
get all the known identifiers.

Some standards, specifically language and country, support 2-character and
3-character alphabetic identifiers, a single `lookup()` function is used to
lookup either.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod codeset;

pub mod country;

pub mod currency;

pub mod language;

pub mod region;

pub mod script;
