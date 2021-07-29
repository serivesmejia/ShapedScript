use std::collections::HashMap;

pub struct Dict<T>(HashMap<String, T>);

impl <T: PartialEq> Dict<T> {
    pub fn new() -> Dict<T> {
        Dict(HashMap::new())
    }

    pub fn add(&mut self, k: &str, v: T) {
        self.0.insert(k.to_string(), v);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains_k(&self, name: &str) -> bool {
        self.0.contains_key(&name.to_string())
    }

    pub fn contains_v(&self, v: &T) -> bool {
        self.0.values().any(|val| val == v)
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.0.get(&name.to_string())
    }
}