use crate::ffi::locale::*;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

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

pub mod api {
    use std::ffi::CStr;
    use std::os::raw;
    use std::ptr;
    use std::str::FromStr;
    use crate::ffi::locale::setlocale;
    use crate::locale::Locale;
    use crate::{LocaleError, LocaleResult};
    use super::*;

    pub fn set_locale_all(new_locale: &Locale) -> bool {
        let category: i32 = Category::all_code() as i32;
        unsafe {
            let c_str: *mut raw::c_char =
                setlocale(category, new_locale.to_string().as_ptr() as *const i8);
            return !(c_str == ptr::null_mut::<raw::c_char>());
        }
    }

    pub fn set_locale(new_locale: &Locale, for_category: Category) -> bool {
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
}

pub mod env {
    use std::env::{var, set_var};
    use std::str::FromStr;
    use crate::locale::Locale;
    use crate::{LocaleError, LocaleResult};
    use super::*;

    pub fn set_locale_all(new_locale: &Locale) -> bool {
        let category = Category::all_name();
        set_var(category, new_locale.to_string());
        true
    }

    pub fn set_locale(new_locale: &Locale, for_category: Category) -> bool {
        let category = for_category.to_os_name();
        set_var(category, new_locale.to_string());
        true
    }

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
    use std::str::FromStr;
    use crate::{Locale, LocaleString};
    use super::{Category, api, env};

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
        println!("test_set_locale");
        let old_setting = api::get_locale(Category::Currency);
        assert_eq!(old_setting.unwrap(), Locale::POSIX);
        let result = api::set_locale(&locale, Category::Currency);
        assert_eq!(result, true);
        let new_setting = api::get_locale(Category::Currency);
        assert_eq!(new_setting.unwrap(), locale);
    }
}
