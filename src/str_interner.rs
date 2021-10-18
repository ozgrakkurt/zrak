use std::collections::HashMap;
use std::mem;

pub struct Interner {
    map: HashMap<&'static str, usize>,
    vec: Vec<&'static str>,
    buf: String,
    full: Vec<String>,
}

pub struct IntStr(usize);

impl Interner {
    pub fn with_capacity(cap: usize) -> Interner {
        Interner {
            map: HashMap::new(),
            vec: Vec::new(),
            buf: String::with_capacity(cap),
            full: Vec::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> IntStr {
        if let Some(&idx) = self.map.get(s) {
            return IntStr(idx);
        }
        let s = unsafe { self.alloc(s) };
        let idx = self.map.len();
        self.map.insert(s, idx);
        self.vec.push(s);

        IntStr(idx)
    }

    pub fn lookup(&self, idx: usize) -> Option<&str> {
        self.vec.get(idx).copied()
    }

    unsafe fn alloc(&mut self, s: &str) -> &'static str {
        let cap = self.buf.capacity();
        if cap < self.buf.len() + s.len() {
            let new_cap = (cap.max(s.len()) + 1).next_power_of_two();
            let new_buf = String::with_capacity(new_cap);
            let old_buf = mem::replace(&mut self.buf, new_buf);
            self.full.push(old_buf);
        }

        let interned = {
            let start = self.buf.len();
            self.buf.push_str(s);
            &self.buf[start..]
        };

        &*(interned as *const str)
    }
}
