use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut hashmap : HashMap<char, i64> = HashMap::new();
        for c in s.chars() {
            hashmap.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        }
        for c in t.chars() {
            hashmap.entry(c).and_modify(|counter| *counter -= 1).or_insert(-1);
        }
        hashmap.values().all(|val| *val == 0)
    }
}
