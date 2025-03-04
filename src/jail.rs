use crate::IString;
use std::{
    borrow::Borrow,
    hash::{Hash, Hasher},
    ptr::addr_eq,
};

/// Put string in the interner by calling [Jail::add_ref]. The string will remain interned
/// until all reference have been removed by calling [Jail::remove_ref].
#[derive(Default)]
pub struct Jail(indexmap::IndexMap<Key, u64, fxhash::FxBuildHasher>);

impl Jail {
    pub fn add_ref(&mut self, s: &str) -> IString {
        if let Some((_, key, count)) = self.0.get_full_mut(s) {
            *count += 1;
            return IString(key.0);
        }

        let key = Key::new(s);
        let out = IString(key.0);

        self.0.insert(key, 1);

        out
    }

    pub fn add_ref_opt(&mut self, o: Option<&str>) -> Option<IString> {
        o.map(|s| self.add_ref(s))
    }

    pub fn remove_ref(&mut self, mut s: IString) {
        let (idx, _, count) = self.0.get_full_mut(&*s).expect("Intern str not found");

        s.safe_drop();

        *count -= 1;

        if *count == 0 {
            self.0.swap_remove_index(idx);
        }
    }

    pub fn remove_ref_opt(&mut self, o: Option<IString>) {
        if let Some(s) = o {
            self.remove_ref(s);
        }
    }
}

struct Key(*mut str);

impl Key {
    fn new(s: &str) -> Self {
        Self(Box::into_raw(Box::<str>::from(s)))
    }
}

impl Borrow<*mut str> for Key {
    fn borrow(&self) -> &*mut str {
        &self.0
    }
}

impl Borrow<str> for Key {
    fn borrow(&self) -> &str {
        unsafe { &*self.0 }
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.0) });
    }
}

impl Eq for Key {}

impl Hash for Key {
    fn hash<H: Hasher>(&self, state: &mut H) {
        unsafe { &*self.0 }.hash(state);
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        addr_eq(self.0, other.0)
    }
}

#[test]
fn istring_add_remove_ref() {
    let mut interner = Jail::default();

    let a = interner.add_ref("foo");
    assert_eq!(*interner.0.values().next().unwrap(), 1);

    let b = interner.add_ref(&String::from("foo"));
    assert_eq!(*interner.0.values().next().unwrap(), 2);

    interner.remove_ref(a);
    assert_eq!(*interner.0.values().next().unwrap(), 1);

    interner.remove_ref(b);
    assert!(interner.0.is_empty());
}

#[test]
fn istring_does_not_panic_after_remove_ref() {
    let mut interner = Jail::default();
    let s = interner.add_ref("test");

    interner.remove_ref(s);
}

#[test]
fn istring_eq() {
    let mut interner = Jail::default();
    let a = interner.add_ref("foo");
    let b = interner.add_ref(&String::from("foo"));
    let c = interner.add_ref("bar");

    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(b, c);
    assert_eq!(&*a, "foo");
    assert_eq!(&*b, "foo");
    assert_eq!(&*c, "bar");

    interner.remove_ref(a);
    interner.remove_ref(b);
    interner.remove_ref(c);
}

#[test]
#[should_panic]
fn istring_should_panic_on_drop_without_remove_ref() {
    let mut interner = crate::Jail::default();
    let s = interner.add_ref("test");

    drop(s);
}
