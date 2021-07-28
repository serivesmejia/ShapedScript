pub trait Dict {
    type Element;

    fn add(&mut self, k: &str, v: Self::Element);

    fn is_empty(&self) -> bool;

    fn contains_k(&self, name: &str) -> bool;

    fn contains_v(&self, v: &Self::Element) -> bool;

    fn get(&self, name: &str) -> Option<&Self::Element>;
}