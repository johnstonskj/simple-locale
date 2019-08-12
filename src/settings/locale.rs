use std::env;
use std::ffi::CStr;
use std::os::raw;
use std::ptr;
use std::str::FromStr;

use crate::ffi::locale::*;
use crate::locale::Locale;
use crate::{LocaleError, LocaleResult};

#[derive(Debug)]
pub enum Category {
    CharacterTypes,
    Currency,
    Message,
    Numeric,
    StringCollation,
    Time,
}

impl Category {
    pub fn all_code() -> u32 {
        LC_ALL
    }

    pub fn all_name() -> &'static str {
        "LC_ALL"
    }

    pub fn to_os_code(&self) -> u32 {
        match self {
            Category::StringCollation => LC_COLLATE,
            Category::CharacterTypes => LC_CTYPE,
            Category::Currency => LC_MONETARY,
            Category::Numeric => LC_NUMERIC,
            Category::Time => LC_TIME,
            Category::Message => LC_MESSAGES,
        }
    }

    pub fn to_os_name(&self) -> &str {
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

pub fn set_locale_all(new_locale: Locale) -> bool {
    let category: i32 = Category::all_code() as i32;
    unsafe {
        let c_str: *mut raw::c_char =
            setlocale(category, new_locale.to_string().as_ptr() as *const i8);
        return !(c_str == ptr::null_mut::<raw::c_char>());
    }
}

pub fn set_locale(new_locale: Locale, for_category: Category) -> bool {
    let category = for_category.to_os_code() as i32;
    unsafe {
        let c_str: *mut raw::c_char =
            setlocale(category, new_locale.to_string().as_ptr() as *const i8);
        return !(c_str == ptr::null_mut::<raw::c_char>());
    }
}

pub fn get_locale(for_category: Category) -> LocaleResult<Locale> {
    let category = for_category.to_os_code() as i32;
    unsafe {
        let c_str: *mut raw::c_char = setlocale(category, ptr::null());
        if c_str == ptr::null_mut::<raw::c_char>() {
            Err(LocaleError::Unsupported)
        } else {
            let r_str = CStr::from_ptr(c_str).to_string_lossy().into_owned();
            Ok(Locale::from_str(&r_str).unwrap())
        }
    }
}

pub fn get_locale_from_env(for_category: Category) -> LocaleResult<Locale> {
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn get_from_env(category: &str) -> Option<String> {
    match env::var(category) {
        Ok(locale) => Some(locale),
        _ => None,
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_locale() {
        println!("test_get_locale");
        let result = get_locale(Category::Currency);
        println!("{:#?}", result);
    }

    #[test]
    fn test_get_locale_from_env() {
        println!("test_get_locale_from_env");
        let result = get_locale_from_env(Category::Currency);
        println!("{:#?}", result);
    }
}
