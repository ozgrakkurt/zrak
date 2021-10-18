use std::collections::HashMap;
use std::mem;

pub struct Interner {
    map: HashMap<&'static str, usize>,
    vec: Vec<&'static str>,
    buf: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IntStr(usize);

impl Interner {
    pub fn new() -> Interner {
        Interner {
            map: HashMap::new(),
            vec: Vec::new(),
            buf: Vec::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> IntStr {
        if let Some(&idx) = self.map.get(s) {
            return IntStr(idx);
        }
        let string = s.to_owned();
        let s = unsafe { mem::transmute::<&str, &'static str>(&string) };
        self.buf.push(string);
        let idx = self.map.len();
        self.map.insert(s, idx);
        self.vec.push(s);

        IntStr(idx)
    }

    pub fn lookup(&self, s: IntStr) -> Option<&str> {
        self.vec.get(s.0).copied()
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
