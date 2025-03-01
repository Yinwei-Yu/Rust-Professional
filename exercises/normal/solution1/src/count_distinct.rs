use std::collections::{HashSet};

pub fn new_count_distinct(input_str: &str) -> usize {
    let input_str=input_str.split(',');
    let mut h :HashSet<&str>=HashSet::new();
    for ch in input_str {
        h.insert(ch);
    }
    h.len()
}
