/*!
Provides ability to get/set the current process locale.

This module allows the client to fetch and set locales for different
`Category` values, or for all.
*/

use crate::ffi::*;
use crate::locale::Locale;
use crate::{LocaleError, LocaleResult};
use std::ffi::CStr;
use std::os::raw;
use std::ptr;
use std::str::FromStr;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// The different categories for which locale information may be
/// set. This implies that entirely different locales may be then
///specified for each category.
#[derive(Debug)]
pub enum Category {
    /// Affects the manner in which characters are classified by
    /// functions such as `isdigit` and so forth.
    CharacterTypes,
    /// Affects the manner in which currency data is formatted.
    Currency,
    /// Affects the display of messages.
    Message,
    /// Affects the manner in which numeric data is formatted.
    Numeric,
    /// Affects the manner in which strings are collated/sorted.
    StringCollation,
    /// Affects the manner in which date/time data is formatted.
    Time,
}

impl Category {
    pub(crate) fn all_code() -> u32 {
        LC_ALL
    }

    pub(crate) fn to_os_code(&self) -> u32 {
        match self {
            Category::StringCollation => LC_COLLATE,
            Category::CharacterTypes => LC_CTYPE,
            Category::Currency => LC_MONETARY,
            Category::Numeric => LC_NUMERIC,
            Category::Time => LC_TIME,
            Category::Message => LC_MESSAGES,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn to_os_mask(&self) -> u32 {
        match self {
            Category::StringCollation => LC_COLLATE_MASK,
            Category::CharacterTypes => LC_CTYPE_MASK,
            Category::Currency => LC_MONETARY_MASK,
            Category::Numeric => LC_NUMERIC_MASK,
            Category::Time => LC_TIME_MASK,
            Category::Message => LC_MESSAGES_MASK,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

const DEFAULT_LOCALE: &str = "";
const QUERY_LOCALE: *const raw::c_char = ptr::null();

/// Set all locale categories to `new_locale`.
pub fn set_locale_all(new_locale: &Locale) -> bool {
    set_locale_wrapper(Category::all_code() as i32, &new_locale.to_string())
}

/// Set all locale categories based on the `LC_ALL` and `LANG` environment
/// variables.
pub fn set_locale_all_from_env() -> bool {
    set_locale_wrapper(Category::all_code() as i32, DEFAULT_LOCALE)
}

/// Set the  locale to `new_locale` for the `for_category` category only.
pub fn set_locale(new_locale: &Locale, for_category: &Category) -> bool {
    set_locale_wrapper(for_category.to_os_code() as i32, &new_locale.to_string())
}

/// Set the  locale for the `for_category` category based on the value
/// of the `LC_{category}` environment variables.
pub fn set_locale_from_env(for_category: &Category) -> bool {
    set_locale_wrapper(for_category.to_os_code() as i32, DEFAULT_LOCALE)
}

/// Get the  locale for the `for_category` category only.
pub fn get_locale(for_category: &Category) -> LocaleResult<Locale> {
    let category = for_category.to_os_code() as i32;
    unsafe {
        let c_str: *mut raw::c_char = setlocale(category, QUERY_LOCALE);
        debug!("setlocale({}, null) returned {:#?}", category, c_str);
        if c_str == ptr::null_mut::<raw::c_char>() {
            Err(LocaleError::OSError)
        } else {
            let r_str = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            Ok(Locale::from_str(&r_str).unwrap())
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn set_locale_wrapper(category: i32, new_locale: &str) -> bool {
    // this is a nice wrapper around the FFI function, it only really
    // does type transformation, logging, and error handling.
    unsafe {
        let c_str: *mut raw::c_char = setlocale(category, new_locale.as_ptr() as *const i8);
        debug!(
            "setlocale({}, {:#?}) returned {:#?}",
            category, new_locale, c_str
        );
        return !(c_str == ptr::null_mut::<raw::c_char>());
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Locale, LocaleString};
    use std::str::FromStr;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_locale() {
        set_locale_all(&Locale::POSIX);
        for category in [
            Category::CharacterTypes,
            Category::Currency,
            Category::Message,
            Category::Numeric,
            Category::StringCollation,
            Category::Time,
        ]
        .iter()
        {
            let result = get_locale(category);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Locale::POSIX);
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_set_locale_all() {
        set_locale_all(&Locale::POSIX);
        for category in [
            Category::CharacterTypes,
            Category::Currency,
            Category::Message,
            Category::Numeric,
            Category::StringCollation,
            Category::Time,
        ]
        .iter()
        {
            let result = get_locale(category);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Locale::POSIX);
        }
    }

    #[test]
    fn test_set_locale_one() {
        // set everything
        set_locale_all(&Locale::POSIX);

        // re-set currency
        let locale = Locale::String(LocaleString::from_str("en_US.UTF-8").unwrap());
        let result = set_locale(&locale, &Category::Currency);
        assert_eq!(result, true);

        // check currency is set correctly
        let new_setting = get_locale(&Category::Currency);
        assert_eq!(new_setting.unwrap(), locale);

        // check everything else is left as-was
        for category in [
            Category::CharacterTypes,
            Category::Message,
            Category::Numeric,
            Category::StringCollation,
            Category::Time,
        ]
        .iter()
        {
            let result = get_locale(category);
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), Locale::POSIX);
        }
    }
}
