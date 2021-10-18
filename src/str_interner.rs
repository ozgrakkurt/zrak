use std::collections::HashMap;
use std::mem;

pub struct Interner {
    map: HashMap<&'static str, usize>,
    vec: Vec<Option<String>>,
    deleted: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntStr(usize);

impl Interner {
    pub fn new() -> Interner {
        Interner {
            map: HashMap::new(),
            vec: Vec::new(),
            deleted: Vec::new(),
        }
    }

    pub fn intern(&mut self, string: String) -> IntStr {
        if let Some(&idx) = self.map.get(string.as_str()) {
            return IntStr(idx);
        }
        let s = unsafe { mem::transmute::<&str, &'static str>(&string) };
        let idx = self.alloc(string);
        self.map.insert(s, idx);

        IntStr(idx)
    }

    pub fn intern_str(&mut self, s: &str) -> IntStr {
        self.intern(s.into())
    }

    pub fn lookup(&self, s: IntStr) -> Option<&str> {
        self.vec
            .get(s.0)
            .map(Option::as_ref)
            .flatten()
            .map(String::as_str)
    }

    pub fn remove(&mut self, s: IntStr) -> Option<String> {
        let idx = s.0;
        if let Some(slot) = self.vec.get_mut(idx) {
            if let Some(s) = slot.take() {
                self.deleted.push(idx);
                self.map.remove(s.as_str()).unwrap();
                return Some(s);
            }
        }

        None
    }

    fn alloc(&mut self, s: String) -> usize {
        if let Some(idx) = self.deleted.pop() {
            self.vec[idx] = Some(s);
            return idx;
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
        let s1 = interner.intern_str("hello");
        let s2 = interner.intern_str("hello");
        let s3 = interner.intern_str("world");
        assert_eq!("hello", interner.lookup(s1).unwrap());
        assert_eq!("hello", interner.lookup(s2).unwrap());
        assert_eq!("world", interner.lookup(s3).unwrap());
        assert_eq!(s1, s2);
        assert_ne!(s1, s3);
        assert_ne!(s2, s3);
    }
}
