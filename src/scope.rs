use crate::LocaleResult;
use crate::string::LocaleString;

pub trait LocaleScope {
    fn get_locale() -> LocaleResult<LocaleString>;

    fn set_locale(locale: LocaleString) -> LocaleResult<()>;

    fn get_settings<T>() -> LocaleResult<T>;
}
