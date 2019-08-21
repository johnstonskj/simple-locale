/*!
Provides ability to get/set the current process locale.

This module allows the client to `get_locale` as well as to
`set_locale`, and `set_locale_all`. There are two implementations
that are related by the operating system, however some processes
tend to only rely on one or the other. The first implementation,
`api`, strictly uses the POSIX C API only whereas the second one,
`env`, strictly uses the set of `LC_*` (and `LANG`) environment
variables only.
*/

use crate::ffi::locale::*;
//use crate::ffi::xlocale::{
//    LC_COLLATE_MASK, LC_CTYPE_MASK, LC_MESSAGES_MASK, LC_MONETARY_MASK, LC_NUMERIC_MASK,
//    LC_TIME_MASK,
//};

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

    pub(crate) fn all_name() -> &'static str {
        "LC_ALL"
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

    //    pub(crate) fn to_os_mask(&self) -> u32 {
    //        match self {
    //            Category::StringCollation => LC_COLLATE_MASK,
    //            Category::CharacterTypes => LC_CTYPE_MASK,
    //            Category::Currency => LC_MONETARY_MASK,
    //            Category::Numeric => LC_NUMERIC_MASK,
    //            Category::Time => LC_TIME_MASK,
    //            Category::Message => LC_MESSAGES_MASK,
    //        }
    //    }

    pub(crate) fn to_os_name(&self) -> &str {
        match self {
            Category::StringCollation => "LC_COLLATE",
            Category::CharacterTypes => "LC_CTYPE",
            Category::Currency => "LC_MONETARY",
            Category::Numeric => "LC_NUMERIC",
            Category::Time => "LC_TIME",
            Category::Message => "LC_MESSAGES",
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

/// Get and set the current locale using the POSIX C API.
pub mod api {
    use super::*;
    use crate::ffi::locale::setlocale;
    use crate::locale::Locale;
    use crate::{LocaleError, LocaleResult};
    use std::ffi::CStr;
    use std::os::raw;
    use std::ptr;
    use std::str::FromStr;

    /// Set all locale categories to `new_locale`.
    pub fn set_locale_all(new_locale: &Locale) -> bool {
        let category: i32 = Category::all_code() as i32;
        unsafe {
            let c_str: *mut raw::c_char =
                setlocale(category, new_locale.to_string().as_ptr() as *const i8);
            println!(
                "setlocale(\"{}\") returned {:#?}",
                new_locale.to_string(),
                c_str
            );
            return !(c_str == ptr::null_mut::<raw::c_char>());
        }
    }

    /// Set the  locale to `new_locale` for the `for_category` category only.
    pub fn set_locale(new_locale: &Locale, for_category: Category) -> bool {
        let category = for_category.to_os_code() as i32;
        unsafe {
            let c_str: *mut raw::c_char =
                setlocale(category, new_locale.to_string().as_ptr() as *const i8);
            println!(
                "setlocale(\"{}\") returned {:#?}",
                new_locale.to_string(),
                c_str
            );
            return !(c_str == ptr::null_mut::<raw::c_char>());
        }
    }

    /// Get the  locale for the `for_category` category only.
    pub fn get_locale(for_category: Category) -> LocaleResult<Locale> {
        let category = for_category.to_os_code() as i32;
        unsafe {
            let c_str: *mut raw::c_char = setlocale(category, ptr::null());
            println!("setlocale(null) returned {:#?}", c_str);
            if c_str == ptr::null_mut::<raw::c_char>() {
                Err(LocaleError::Unsupported)
            } else {
                let r_str = CStr::from_ptr(c_str).to_string_lossy().into_owned();
                Ok(Locale::from_str(&r_str).unwrap())
            }
        }
    }
}

/// Get and set the current locale using LC_* environment variables.
pub mod env {
    use super::*;
    use crate::locale::Locale;
    use crate::{LocaleError, LocaleResult};
    use std::env::{set_var, var};
    use std::str::FromStr;

    /// Set all locale categories to `new_locale`.
    pub fn set_locale_all(new_locale: &Locale) -> bool {
        let category = Category::all_name();
        set_var(category, new_locale.to_string());
        true
    }

    /// Set the  locale to `new_locale` for the `for_category` category only.
    pub fn set_locale(new_locale: &Locale, for_category: Category) -> bool {
        let category = for_category.to_os_name();
        set_var(category, new_locale.to_string());
        true
    }

    /// Get the  locale for the `for_category` category only.
    pub fn get_locale(for_category: Category) -> LocaleResult<Locale> {
        let category = for_category.to_os_name();
        let mut locale_str = get_from_env(category);
        if let None = locale_str {
            locale_str = get_from_env(Category::all_name());
        }
        match locale_str {
            Some(locale) => match Locale::from_str(&locale) {
                Ok(locale) => Ok(locale),
                Err(e) => {
                    warn!("locale parse error: {:?}", e);
                    Err(LocaleError::InvalidLocaleString)
                }
            },
            None => Err(LocaleError::UnsetCategory),
        }
    }

    // --------------------------------------------------------------------------------------------
    // Private Functions
    // --------------------------------------------------------------------------------------------

    fn get_from_env(category: &str) -> Option<String> {
        match var(category) {
            Ok(locale) => Some(locale),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{api, env, Category};
    use crate::{Locale, LocaleString};
    use std::str::FromStr;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_locale() {
        println!("test_get_locale");
        let result = api::get_locale(Category::Currency);
        println!("{:#?}", result);
    }

    #[test]
    fn test_get_locale_from_env() {
        println!("test_get_locale_from_env");
        let result = env::get_locale(Category::Currency);
        println!("{:#?}", result);
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_set_locale() {
        let locale = Locale::String(LocaleString::from_str("en_US.UTF-8").unwrap());
        println!("test_set_locale to {:#?}", locale);
        let old_setting = api::get_locale(Category::Currency);
        println!("locale was {:#?}", old_setting);
        let result = api::set_locale(&locale, Category::Currency);
        assert_eq!(result, true);
        let new_setting = api::get_locale(Category::Currency);
        assert_eq!(new_setting.unwrap(), locale);
    }
}
