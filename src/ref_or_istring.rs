use crate::IString;
use std::{
    borrow::Borrow,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

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
        **self == **other
    }
}
