/*!
Fetch locale-specific date and time formatting settings.

This module provides the details for both time and date formatting
as well as a set of calendar names used for day and month display.

## Date and Time Formatting

The date and time formatting strings use the field specifiers from
the C [`strftime`](https://man.openbsd.org/strftime.3) function. These
strings may also be used with the chrono crate's
[`format::strftime`](https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html)
module.
*/

use crate::ffi::langinfo;
use crate::ffi::utils::*;
use crate::{Locale, LocaleResult};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// This structure captures a name and its abbreviated form.
#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    /// Full name
    name: Option<String>,
    /// Abbreviated name
    abbreviated: Option<String>,
}

/// The set of all calendar names.
#[derive(Debug, Clone, PartialEq)]
pub struct CalendarNames {
    /// Names of the days of the week (Sunday .. Saturday)
    week_day_names: Vec<Name>,
    /// Names of the months in a year (January .. December)
    month_names: Vec<Name>,
    /// The string for AM time
    am_string: Option<String>,
    /// The string for PM time
    pm_string: Option<String>,
}

/// Determines in general whether days are displayed before, or
/// after, months.
#[derive(Debug, Clone, PartialEq)]
pub enum MonthDayOrder {
    /// Display month, then day
    MonthDay,
    /// Display day, then month
    DayMonth,
}

/// The complete date and time formatting information.
#[derive(Debug, Clone, PartialEq)]
pub struct DateTimeFormat {
    /// The string to use to format a complete date and time.
    date_time_format: Option<String>,
    /// The string to use to format a date.
    date_format: Option<String>,
    /// The string to use to format a time, in 24 hours.
    time_format: Option<String>,
    /// The string to use to format a time, with am/pm.
    time_ampm_format: Option<String>,
    era: Option<String>,
    era_date_time_format: Option<String>,
    era_date_format: Option<String>,
    era_time_format: Option<String>,
    /// The locale's preference on date and month ordering.
    month_day_order: Option<MonthDayOrder>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Fetch calendar names for days and months.
pub fn get_calendar_names() -> CalendarNames {
    CalendarNames {
        week_day_names: make_name_vector(7, langinfo::DAY_1, langinfo::ABDAY_1),
        month_names: make_name_vector(12, langinfo::MON_1, langinfo::ABMON_1),
        am_string: get_nl_string(langinfo::AM_STR),
        pm_string: get_nl_string(langinfo::PM_STR),
    }
}

/// Fetch the calendar names for a specified `Locale`.
///
/// # Arguments
///
/// * `locale` - The locale to query.
/// * `inherit_current` - Whether the specified locale should inherit
///   from the current locale.
///
/// If `inherit_current` is `false` the `locale` specified will be treated
/// as an entirely new and complete locale when calling the C
/// [`newlocale`](https://man.openbsd.org/newlocale.3) function. If it is
/// `true` the `locale` is assumed to be a partially specified one and inherits
/// any unspecified components from the current locale. For example, if the
/// current locale is `en_US.UTF-8` and the parameters passed are `_NZ` and
/// `true` then the resulting locale will be `en_NZ.UTF-8`.
pub fn get_calendar_names_for_locale(
    locale: Locale,
    inherit_current: bool,
) -> LocaleResult<CalendarNames> {
    get_format_for_locale(locale, &get_calendar_names, inherit_current)
}

/// Fetch the date and time formatting settings for the current locale.
pub fn get_date_time_format() -> DateTimeFormat {
    DateTimeFormat {
        date_time_format: get_nl_string(langinfo::D_T_FMT),
        date_format: get_nl_string(langinfo::D_FMT),
        time_format: get_nl_string(langinfo::T_FMT),
        time_ampm_format: get_nl_string(langinfo::T_FMT_AMPM),
        era: get_nl_string(langinfo::ERA),
        era_date_time_format: get_nl_string(langinfo::ERA_D_T_FMT),
        era_date_format: get_nl_string(langinfo::ERA_D_FMT),
        era_time_format: get_nl_string(langinfo::ERA_T_FMT),
        month_day_order: match get_nl_string(langinfo::D_MD_ORDER) {
            Some(s) => {
                if s == "md".to_string() {
                    Some(MonthDayOrder::MonthDay)
                } else if s == "dm".to_string() {
                    Some(MonthDayOrder::DayMonth)
                } else {
                    None
                }
            }
            _ => None,
        },
    }
}

/// Fetch the date and time formatting rules for a specified `Locale`.
///
/// # Arguments
///
/// * `locale` - The locale to query.
/// * `inherit_current` - Whether the specified locale should inherit
///   from the current locale.
///
/// If `inherit_current` is `false` the `locale` specified will be treated
/// as an entirely new and complete locale when calling the C
/// [`newlocale`](https://man.openbsd.org/newlocale.3) function. If it is
/// `true` the `locale` is assumed to be a partially specified one and inherits
/// any unspecified components from the current locale. For example, if the
/// current locale is `en_US.UTF-8` and the parameters passed are `_NZ` and
/// `true` then the resulting locale will be `en_NZ.UTF-8`.
pub fn get_date_time_format_for_locale(
    locale: Locale,
    inherit_current: bool,
) -> LocaleResult<DateTimeFormat> {
    get_format_for_locale(locale, &get_date_time_format, inherit_current)
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn make_name_vector(count: u32, n_st: u32, ab_st: u32) -> Vec<Name> {
    (0..count)
        .map(|offset| Name {
            name: get_nl_string(n_st + offset),
            abbreviated: get_nl_string(ab_st + offset),
        })
        .collect()
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{get_calendar_names, get_date_time_format};
    use crate::settings::locale::api::get_locale;
    use crate::settings::locale::Category;
    use crate::settings::time::{MonthDayOrder, Name};
    use crate::Locale;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_week_day_names() {
        let locale = get_locale(Category::Time).unwrap();
        if locale == Locale::POSIX {
            let names = get_calendar_names();
            assert_eq!(names.week_day_names.len(), 7);
            let sunday = names.week_day_names.get(0).unwrap();
            assert_eq!(
                sunday,
                &Name {
                    name: Some("Sunday".to_string()),
                    abbreviated: Some("Sun".to_string())
                }
            );
        } else {
            panic!("expecting POSIX locale");
        }
    }

    #[test]
    fn test_month_names() {
        let locale = get_locale(Category::Time).unwrap();
        if locale == Locale::POSIX {
            let names = get_calendar_names();
            assert_eq!(names.month_names.len(), 12);
            let january = names.month_names.get(0).unwrap();
            assert_eq!(
                january,
                &Name {
                    name: Some("January".to_string()),
                    abbreviated: Some("Jan".to_string())
                }
            );
        } else {
            panic!("expecting POSIX locale");
        }
    }

    #[test]
    fn test_ampm_names() {
        let locale = get_locale(Category::Time).unwrap();
        if locale == Locale::POSIX {
            let names = get_calendar_names();
            assert_eq!(names.am_string, Some("AM".to_string()));
            assert_eq!(names.pm_string, Some("PM".to_string()));
        } else {
            panic!("expecting POSIX locale");
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_date_time_formats() {
        let locale = get_locale(Category::Time).unwrap();
        if locale == Locale::POSIX {
            let formats = get_date_time_format();
            println!("{:#?}", formats);
            assert_eq!(formats.date_format, Some("%m/%d/%y".to_string()));
            assert_eq!(formats.era, None);
            assert_eq!(formats.month_day_order, Some(MonthDayOrder::MonthDay));
        } else {
            panic!("expecting POSIX locale");
        }
    }
}
