use crate::IString;
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};
use str_utils::{
    cmp::{EqExt, OrdExt},
    str_ci::StrCi,
};

/// A [std::borrow::Cow] alternative for holding IString or str but with case insensitive compare.
pub enum RefOrIStringCi<'a> {
    Ref(&'a str),
    IString(IString),
}

impl Borrow<StrCi> for RefOrIStringCi<'_> {
    #[inline]
    fn borrow(&self) -> &StrCi {
        StrCi::new(self)
    }
}

impl Debug for RefOrIStringCi<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&**self, f)
    }
}

impl Deref for RefOrIStringCi<'_> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        match self {
            Self::IString(s) => s,
            Self::Ref(s) => s,
        }
    }
}

impl Display for RefOrIStringCi<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&**self, f)
    }
}

impl Eq for RefOrIStringCi<'_> {}

impl<'a> From<&'a str> for RefOrIStringCi<'a> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::Ref(value)
    }
}

impl From<IString> for RefOrIStringCi<'_> {
    #[inline]
    fn from(value: IString) -> Self {
        Self::IString(value)
    }
}

impl<'a> From<RefOrIStringCi<'a>> for Option<IString> {
    #[inline]
    fn from(value: RefOrIStringCi<'a>) -> Self {
        match value {
            RefOrIStringCi::IString(v) => Some(v),
            RefOrIStringCi::Ref(_) => None,
        }
    }
}

impl Hash for RefOrIStringCi<'_> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.hash_ci(state);
    }
}

impl Ord for RefOrIStringCi<'_> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_ci(other)
    }
}

impl PartialEq for RefOrIStringCi<'_> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.eq_ci(other)
    }
}

impl PartialEq<str> for RefOrIStringCi<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.eq_ci(other)
    }
}

impl PartialEq<RefOrIStringCi<'_>> for str {
    #[inline]
    fn eq(&self, other: &RefOrIStringCi<'_>) -> bool {
        self.eq_ci(other)
    }
}

impl PartialOrd for RefOrIStringCi<'_> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for RefOrIStringCi<'_> {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.cmp_ci(other))
    }
}

impl PartialOrd<RefOrIStringCi<'_>> for str {
    #[inline]
    fn partial_cmp(&self, other: &RefOrIStringCi<'_>) -> Option<Ordering> {
        Some(self.cmp_ci(other))
    }
}
