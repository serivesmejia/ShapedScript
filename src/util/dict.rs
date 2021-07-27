pub trait Dict {
    type Element;

    pub fn add(&mut self, k: &str, v: Self::Element);

    pub fn is_empty(&self) -> bool;

    pub fn contains_k(name: &str) -> bool;

    pub fn contains_v(v: Self::Element) -> bool;

    pub fn get(&self, name: &str) -> Option<&T>;
}