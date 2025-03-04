use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt::{self, Debug, Display, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
    ptr::addr_eq,
    sync::OnceLock,
};

/// An interned string
///
/// # Note
///
/// An IStr cannot be cloned nor copy because it needs to increment / decrement counter
/// in the [crate::Interner] by calling [add_ref](crate::Interner::add_ref) or [remove_ref](crate::Interner::remove_ref).
///
/// # Panic
///
/// It will panic on drop, the user must call [remove_ref](crate::Interner::remove_ref) to correctly dispose of it.
pub struct IString(pub(crate) *mut str);

impl IString {
    pub(crate) fn safe_drop(&mut self) {
        self.0 = null_str_ptr();
    }
}

impl Borrow<str> for IString {
    #[inline]
    fn borrow(&self) -> &str {
        unsafe { &*self.0 }
    }
}

impl Debug for IString {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(unsafe { &*self.0 }, f)
    }
}

impl Deref for IString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

impl Display for IString {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(unsafe { &*self.0 }, f)
    }
}

impl Drop for IString {
    fn drop(&mut self) {
        if !addr_eq(self.0, null_str_ptr()) {
            panic!("StrInt leaked")
        }
    }
}

impl Eq for IString {}

impl Hash for IString {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { &*self.0 }.hash(state);
    }
}

impl Ord for IString {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl PartialEq for IString {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        addr_eq(self.0, other.0)
    }
}

impl PartialEq<str> for IString {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        *unsafe { &*self.0 } == *other
    }
}

impl PartialEq<IString> for str {
    #[inline]
    fn eq(&self, other: &IString) -> bool {
        *self == *unsafe { &*other.0 }
    }
}

impl PartialOrd for IString {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn null_str_ptr() -> *mut str {
    static ONCE: OnceLock<StrNullPtr> = OnceLock::new();
    ONCE.get_or_init(StrNullPtr::new).0
}

struct StrNullPtr(*mut str);

impl StrNullPtr {
    fn new() -> Self {
        Self(Box::into_raw(String::from("NULLPTR").into_boxed_str()))
    }
}

unsafe impl Send for StrNullPtr {}
unsafe impl Sync for StrNullPtr {}
