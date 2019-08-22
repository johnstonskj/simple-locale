/*!
Bindings to POSIX API for locale data.

All submodules of `ffi` are generated with bindgen using a wrapper script
`create-bindings.sh` at the root of the Git repo.

**Do not modify these modules directly by hand.**
*/

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
mod _macos {
    pub use super::macos::langinfo::{nl_item, nl_langinfo,};
    pub use super::macos::langinfo::{
        CODESET,
        D_T_FMT,
        D_FMT,
        T_FMT,
        T_FMT_AMPM,
        AM_STR,
        PM_STR,
        DAY_1,
        DAY_2,
        DAY_3,
        DAY_4,
        DAY_5,
        DAY_6,
        DAY_7,
        ABDAY_1,
        ABDAY_2,
        ABDAY_3,
        ABDAY_4,
        ABDAY_5,
        ABDAY_6,
        ABDAY_7,
        MON_1,
        MON_2,
        MON_3,
        MON_4,
        MON_5,
        MON_6,
        MON_7,
        MON_8,
        MON_9,
        MON_10,
        MON_11,
        MON_12,
        ABMON_1,
        ABMON_2,
        ABMON_3,
        ABMON_4,
        ABMON_5,
        ABMON_6,
        ABMON_7,
        ABMON_8,
        ABMON_9,
        ABMON_10,
        ABMON_11,
        ABMON_12,
        ERA,
        ERA_D_FMT,
        ERA_D_T_FMT,
        ERA_T_FMT,
        ALT_DIGITS,
        RADIXCHAR,
        THOUSEP,
        YESEXPR,
        NOEXPR,
        YESSTR,
        NOSTR,
        CRNCYSTR,
        D_MD_ORDER,
    };

    pub use super::macos::locale::{lconv, localeconv, setlocale};
    pub use super::macos::locale::{
        LC_ALL, LC_COLLATE, LC_CTYPE, LC_MESSAGES, LC_MONETARY, LC_NUMERIC, LC_TIME,
    };

    pub use super::macos::xlocale::{locale_t, newlocale, duplocale, freelocale, uselocale,  ___mb_cur_max};
    pub use super::macos::xlocale::{
        LC_ALL_MASK, LC_COLLATE_MASK, LC_CTYPE_MASK, LC_MESSAGES_MASK, LC_MONETARY_MASK, LC_NUMERIC_MASK,
        LC_TIME_MASK,
    };

    use super::linux::stdlib::__ctype_get_mb_cur_max;
    pub const ___mb_cur_max: unsafe extern "C" fn() -> usize = __ctype_get_mb_cur_max;
}
#[cfg(target_os = "macos")]
pub(crate) use _macos::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
mod _linux {
    pub use super::linux::langinfo::{nl_item, nl_langinfo};
    pub use super::linux::langinfo::{
        _NL_CTYPE_CODESET_NAME as CODESET,
        D_T_FMT,
        D_FMT,
        T_FMT,
        T_FMT_AMPM,
        AM_STR,
        PM_STR,
        DAY_1,
        DAY_2,
        DAY_3,
        DAY_4,
        DAY_5,
        DAY_6,
        DAY_7,
        ABDAY_1,
        ABDAY_2,
        ABDAY_3,
        ABDAY_4,
        ABDAY_5,
        ABDAY_6,
        ABDAY_7,
        MON_1,
        MON_2,
        MON_3,
        MON_4,
        MON_5,
        MON_6,
        MON_7,
        MON_8,
        MON_9,
        MON_10,
        MON_11,
        MON_12,
        ABMON_1,
        ABMON_2,
        ABMON_3,
        ABMON_4,
        ABMON_5,
        ABMON_6,
        ABMON_7,
        ABMON_8,
        ABMON_9,
        ABMON_10,
        ABMON_11,
        ABMON_12,
        ERA,
        ERA_D_FMT,
        ERA_D_T_FMT,
        ERA_T_FMT,
        ALT_DIGITS,
        RADIXCHAR,
        THOUSEP,
        __YESEXPR as YESEXPR,
        __NOEXPR as NOEXPR,
        __YESSTR as YESSTR,
        __NOSTR as NOSTR,
        _NL_MONETARY_CRNCYSTR as CRNCYSTR,
    };
    pub use super::linux::locale::{lconv, localeconv, locale_t, setlocale, newlocale, duplocale, freelocale, uselocale};

    pub use super::linux::locale::{
        LC_ALL, LC_COLLATE, LC_CTYPE, LC_MESSAGES, LC_MONETARY, LC_NUMERIC, LC_TIME,
        LC_ALL_MASK, LC_COLLATE_MASK, LC_CTYPE_MASK, LC_MESSAGES_MASK, LC_MONETARY_MASK, LC_NUMERIC_MASK,
        LC_TIME_MASK,
    };

    extern "C" { fn __ctype_get_mb_cur_max() -> usize ; }
    #[allow(non_upper_case_globals)]
    pub const ___mb_cur_max: unsafe extern "C" fn() -> usize = __ctype_get_mb_cur_max;
}
#[cfg(target_os = "linux")]
pub(crate) use _linux::*;

pub(crate) mod utils;
