use crate::IString;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

/// A [std::borrow::Cow] alternative for holding IString or str.
pub enum RefOrIString<'a> {
    Ref(&'a str),
    IString(IString),
}

impl Borrow<str> for RefOrIString<'_> {
    #[inline]
    fn borrow(&self) -> &str {
        self
    }
}

impl Debug for RefOrIString<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl Deref for RefOrIString<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::IString(s) => s,
            Self::Ref(s) => s,
        }
    }
}

impl Display for RefOrIString<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl Eq for RefOrIString<'_> {}

impl<'a> From<&'a str> for RefOrIString<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::Ref(value)
    }
}

impl From<IString> for RefOrIString<'_> {
    #[inline]
    fn from(value: IString) -> Self {
        Self::IString(value)
    }
}

impl<'a> From<RefOrIString<'a>> for Option<IString> {
    #[inline]
    fn from(value: RefOrIString<'a>) -> Self {
        match value {
            RefOrIString::IString(v) => Some(v),
            RefOrIString::Ref(_) => None,
        }
    }
}

impl Hash for RefOrIString<'_> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        (**self).hash(state);
    }
}

impl PartialEq for RefOrIString<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::IString(a), Self::IString(b)) => a == b,
            (Self::IString(a), Self::Ref(b)) => a == *b,
            (Self::Ref(a), Self::IString(b)) => *a == b,
            (Self::Ref(a), Self::Ref(b)) => a == b,
        }
    }
}

impl PartialEq<str> for RefOrIString<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        (&**self) == other
    }
}

impl PartialEq<RefOrIString<'_>> for str {
    #[inline]
    fn eq(&self, other: &RefOrIString<'_>) -> bool {
        self == (&**other)
    }
}

impl PartialOrd for RefOrIString<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for RefOrIString<'_> {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some((**self).cmp(other))
    }
}

impl PartialOrd<RefOrIString<'_>> for str {
    #[inline]
    fn partial_cmp(&self, other: &RefOrIString<'_>) -> Option<Ordering> {
        Some(self.cmp(&**other))
    }
}
