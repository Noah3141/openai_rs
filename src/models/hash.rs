use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


pub fn calculate_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}