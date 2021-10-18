use std::collections::HashMap;
use std::mem;

pub struct Interner {
    map: HashMap<&'static str, usize>,
    vec: Vec<String>,
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

    pub fn intern(&mut self, string: String) -> IntStr {
        if let Some(&idx) = self.map.get(string.as_str()) {
            return IntStr(idx);
        }
        let s = unsafe { mem::transmute::<&str, &'static str>(&string) };
        let idx = self.vec.len();
        self.vec.push(string);
        self.map.insert(s, idx);

        IntStr(idx)
    }

    pub fn intern_str(&mut self, s: &str) -> IntStr {
        self.intern(s.into())
    }

    pub fn lookup(&self, s: IntStr) -> Option<&str> {
        self.vec.get(s.0).map(String::as_str)
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
