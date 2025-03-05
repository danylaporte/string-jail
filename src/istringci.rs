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

/// A case insenstive [IString] container. This can be used in hash maps to index strings.
pub struct IStringCi(pub IString);

impl Borrow<StrCi> for IStringCi {
    #[inline]
    fn borrow(&self) -> &StrCi {
        StrCi::new(&self.0)
    }
}

impl Debug for IStringCi {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Deref for IStringCi {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for IStringCi {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Eq for IStringCi {}

impl Hash for IStringCi {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash_ci(state);
    }
}

impl Ord for IStringCi {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp_ci(&other.0)
    }
}

impl PartialEq for IStringCi {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ci(&other.0)
    }
}

impl PartialEq<str> for IStringCi {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ci(other)
    }
}

impl PartialEq<IStringCi> for str {
    #[inline]
    fn eq(&self, other: &IStringCi) -> bool {
        self.eq_ci(&other.0)
    }
}

impl PartialOrd for IStringCi {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<str> for IStringCi {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<Ordering> {
        Some(self.0.cmp_ci(other))
    }
}

impl PartialOrd<IStringCi> for str {
    #[inline]
    fn partial_cmp(&self, other: &IStringCi) -> Option<Ordering> {
        Some(self.cmp_ci(&other.0))
    }
}
