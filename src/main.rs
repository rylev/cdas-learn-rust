use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn main() {
    let mut map = OurHashMap::new();
    map.push("hello".to_string(), 7);
    assert!(map.get("hello".to_string()) == Some(7));
    map.push("Wow".to_string(), 0);
    assert!(map.get("Wow".to_string()) == Some(0));
}

struct OurHashMap {
    buckets: Vec<Option<(String, u8)>>,
}

const SIZE: usize = 10;
impl OurHashMap {
    fn new() -> OurHashMap {
        let mut buckets = Vec::with_capacity(SIZE);
        for _ in 0..10 {
            buckets.push(None);
        }

        OurHashMap { buckets }
    }

    fn push(&mut self, key: String, value: u8) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let index = hasher.finish() as usize % self.buckets.len();
        self.buckets[index] = Some((key, value));
    }

    fn get(&self, key: String) -> Option<u8> {
        // Imperative style
        // for pair in self.buckets {
        //     if pair.0 == key {
        //         return Some(pair.1);
        //     }
        // }

        // None

        // self.buckets
        //     .iter()
        //     .find(|pair| pair.0 == key)
        //     .map(|(_, current_value)| *current_value)
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let index = hasher.finish() as usize % self.buckets.len();
        match self.buckets[index] {
            Some((_, v)) => Some(v),
            None => None,
        }
    }
}
