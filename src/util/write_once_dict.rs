use super::table::Table;
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


}

impl <T> Dict for WriteOnceDict<T> {
    type Element = T;

    pub fn add(&mut self, k: &str, v: T) {
        map.insert(String::from(k), v);
    }

    pub fn is_empty(&self) -> bool {
        map.is_empty();
    }

    pub fn contains_k(name: &str) -> bool {
        map.contains_key(name)
    }

    pub fn contains_v(v: T) {
        map.contains_value(v)
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        map.get(name)
    }
}