pub mod images;
pub mod logs_asserter;
pub mod trace_asserter;
use std::fmt;

pub enum Protocol {
    Tonic,
    Http,
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::Tonic => write!(f, "Tonic"),
            Protocol::Http => write!(f, "Http"),
        }
    }
}
