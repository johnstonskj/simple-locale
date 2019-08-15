use crate::ffi::utils::*;
use crate::ffi::langinfo;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum MonthDayOrder {
    MonthDay,
    DayMonth,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Name {
    name: Option<String>,
    abbreviated: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalendarNames {
    week_day_names: Vec<Name>,
    month_names: Vec<Name>,
    am_string: Option<String>,
    pm_string: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DateTimeFormat {
    date_time_format: Option<String>,
    date_format: Option<String>,
    time_format: Option<String>,
    time_ampm_format: Option<String>,
    era: Option<String>,
    era_date_time_format: Option<String>,
    era_date_format: Option<String>,
    era_time_format: Option<String>,
    month_day_order: Option<MonthDayOrder>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_calendar_names() -> CalendarNames {
    CalendarNames{
        week_day_names: make_name_vector(7, langinfo::DAY_1, langinfo::ABDAY_1),
        month_names: make_name_vector(12, langinfo::MON_1, langinfo::ABMON_1),
        am_string: get_nl_string(langinfo::AM_STR),
        pm_string: get_nl_string(langinfo::PM_STR),
    }
}

fn make_name_vector(count: u32, n_st: u32, ab_st: u32) -> Vec<Name> {
    (0..count).map(|offset|
        Name{
            name: get_nl_string(n_st + offset),
            abbreviated: get_nl_string(ab_st + offset),
        }
    ).collect()
}

pub fn get_date_time_format() -> DateTimeFormat {
    DateTimeFormat{
        date_time_format: get_nl_string(langinfo::D_T_FMT),
        date_format: get_nl_string(langinfo::D_FMT),
        time_format: get_nl_string(langinfo::T_FMT),
        time_ampm_format: get_nl_string(langinfo::T_FMT_AMPM),
        era: get_nl_string(langinfo::ERA),
        era_date_time_format: get_nl_string(langinfo::ERA_D_T_FMT),
        era_date_format: get_nl_string(langinfo::ERA_D_FMT),
        era_time_format: get_nl_string(langinfo::ERA_T_FMT),
        month_day_order: match get_nl_string(langinfo::D_MD_ORDER) {
            Some(s) => if s == "md".to_string() {
                Some(MonthDayOrder::MonthDay)
            } else if s == "dm".to_string() {
                Some(MonthDayOrder::DayMonth)
            } else {
                None
            },
            _ => None,
        },
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{get_date_time_format, get_calendar_names};

    // --------------------------------------------------------------------------------------------
    # [test]
    fn test_something() {
        println!("{:#?}", get_calendar_names());
        println!("{:#?}", get_date_time_format());
    }
}