use super::dict::Dict;
use std::collections::HashMap;

pub struct WriteOnceDict<T> {
    map: HashMap<String, T>
}

impl <T> WriteOnceDict<T> {
    pub fn new() -> WriteOnceDict<T> {
        WriteOnceDict {
            map: HashMap::new()
        }
    }

    fn check(&self, key: &str) {
        if self.map.contains_key(key) {
            panic!("Cannot redefine '{}' in a write once dict", key)
        }
    }

    pub fn keys(&self) -> Vec<String> {
        let mut result = vec!();
        self.map.keys().for_each(|ref k| result.push(k.to_string()));

        result
    }
}

impl <T: PartialEq> Dict for WriteOnceDict<T> {
    type Element = T;

    fn add(&mut self, k: &str, v: T) {
        self.check(k);
        self.map.insert(String::from(k), v);
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn contains_k(&self, name: &str) -> bool {
        self.map.contains_key(name)
    }

    fn contains_v(&self, v: &T) -> bool {
        self.map.values().any(| val | val == v)
    }

    fn get(&self, name: &str) -> Option<&T> {
        self.map.get(name)
    }
}