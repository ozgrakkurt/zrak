use std::collections::HashMap;
use std::mem;

pub struct Interner {
    map: HashMap<&'static str, usize>,
    vec: Vec<&'static str>,
    buf: Vec<String>,
}

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

    pub fn lookup(&self, idx: usize) -> Option<&str> {
        self.vec.get(idx).copied()
    }
}
