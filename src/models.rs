use std::fmt::Display;

pub mod show;
pub mod watched;
pub mod wl;

pub struct ValidatedTitle(String);

impl From<ValidatedTitle> for String {
    fn from(value: ValidatedTitle) -> Self {
        value.0
    }
}

impl Display for ValidatedTitle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
