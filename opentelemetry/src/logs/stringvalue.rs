use crate::StringValue as CommonStringValue;
use std::borrow::Cow;
use std::{fmt, hash};

/// Enum to represent different types of string storage for OpenTelemetry
#[derive(Clone, Debug, Eq)]
enum OtelString<'a> {
    /// Static string slice with a 'static lifetime
    Static(&'static str),
    /// Cow of a str, which can be either a non-static borrowed str or an owned String
    Dynamic(Cow<'a, str>),
}

impl<'a> OtelString<'a> {
    /// Returns the string as a `&str` reference
    fn as_str(&self) -> &str {
        match self {
            OtelString::Static(s) => s,
            OtelString::Dynamic(cow) => cow.as_ref(),
        }
    }
}

impl<'a> PartialOrd for OtelString<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for OtelString<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl<'a> PartialEq for OtelString<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'a> hash::Hash for OtelString<'a> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

/// Wrapper for string-like values
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StringValue<'a>(OtelString<'a>);

impl<'a> fmt::Debug for StringValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> fmt::Display for StringValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}

impl<'a> AsRef<str> for StringValue<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl<'a> StringValue<'a> {
    /// Returns a string slice to this value
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Create a `StringValue` from a static string
    pub const fn from_static(s: &'static str) -> Self {
        StringValue(OtelString::Static(s))
    }
}

// General implementation for non-static string references
impl<'a> From<&'a str> for StringValue<'a> {
    fn from(s: &'a str) -> Self {
        StringValue(OtelString::Dynamic(Cow::Borrowed(s)))
    }
}

// Implementation for converting from owned `String` to `StringValue`
impl<'a> From<String> for StringValue<'a> {
    fn from(s: String) -> Self {
        StringValue(OtelString::Dynamic(Cow::Owned(s)))
    }
}

// Implementation for converting from `Cow<'a, str>` to `StringValue`
impl<'a> From<Cow<'a, str>> for StringValue<'a> {
    fn from(cow: Cow<'a, str>) -> Self {
        StringValue(OtelString::Dynamic(cow))
    }
}
