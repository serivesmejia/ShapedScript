use std::collections::HashMap;

pub struct StringPool {
    pool: HashMap<usize, String>,
    building_str: Vec<char>
}

impl StringPool {

    pub fn new() -> StringPool {
        StringPool {
            pool: HashMap::new(),
            building_str: vec!()
        }
    }

    pub fn new_initial(initial_pool: HashMap<usize, String>) -> StringPool {
        StringPool {
            pool: initial_pool,
            building_str: vec!()
        }
    }

    pub fn push(&mut self, str: String) -> usize {
        for (i, s) in &self.pool {
            if str == *s {
                return *i
            }
        }

        let i = self.pool.len();
        self.pool.insert(i, str);

        i
    }

    pub fn push_i(&mut self, index: usize, str: String) -> usize {
        self.pool.insert(index, str);
        index
    }

    pub fn read(&self, index: usize) -> Option<&String> {
        self.pool.get(&index)
    }

    pub fn concat(&mut self, a_str: usize, b_str: usize) -> usize {
        let mut a = self.read(a_str).unwrap().clone();
        let b = self.read(b_str).unwrap();

        a.push_str(b);
        self.push(a)
    }

    pub fn push_chr(&mut self, chr: char) {
        self.building_str.push(chr);
    }

    pub fn build(&mut self) -> usize {
        let built_str = (&self.building_str).into_iter().collect();
        self.building_str.clear();

        self.push(built_str)
    }

}