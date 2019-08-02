/*!
Bindings to POSIX API for locale data.

All submodules of `ffi` are generated with bindgen using a wrapper script
`create-bindings.sh` at the root of the Git repo. Do not modify these modules
directly by hand.
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

#[cfg(target_os = "macos")]
pub(crate) mod macos;
#[cfg(target_os = "macos")]
pub use macos::{langinfo, localcharset, locale, xlocale};
