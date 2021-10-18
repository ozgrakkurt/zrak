use std::collections::HashMap;
use std::mem;

pub struct Interner {
    map: HashMap<&'static str, usize>,
    vec: Vec<Option<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntStr(usize);

impl Interner {
    pub fn new() -> Interner {
        Interner {
            map: HashMap::new(),
            vec: Vec::new(),
        }
    }

    pub fn intern<S: Into<String>>(&mut self, s: S) -> IntStr {
        let string = s.into();
        if let Some(&idx) = self.map.get(string.as_str()) {
            return IntStr(idx);
        }
        let s = unsafe { mem::transmute::<&str, &'static str>(&string) };
        let idx = unsafe { self.alloc(string) };
        self.map.insert(s, idx);

        IntStr(idx)
    }

    pub fn lookup(&self, s: IntStr) -> Option<&str> {
        self.vec
            .get(s.0)
            .map(Option::as_ref)
            .flatten()
            .map(String::as_str)
    }

    pub fn delete(&mut self, s: IntStr) -> Option<()> {
        if let Some(slot) = self.vec.get_mut(s.0) {
            if let Some(s) = slot.take() {
                self.map.remove(s.as_str()).unwrap();
                return Some(());
            }
        }

        None
    }

    unsafe fn alloc(&mut self, s: String) -> usize {
        for (i, slot) in self.vec.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = Some(s);
                return i;
            }
        }
        self.vec.push(Some(s));
        self.vec.len() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interner() {
        let mut interner = Interner::new();
        let s1 = interner.intern("hello");
        let s2 = interner.intern("hello");
        let s3 = interner.intern("world");
        assert_eq!("hello", interner.lookup(s1).unwrap());
        assert_eq!("hello", interner.lookup(s2).unwrap());
        assert_eq!("world", interner.lookup(s3).unwrap());
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
        assert_ne!(s2, s3);
    }
}
