use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagrams : HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for string in strs {
            let mut sorted_string : Vec<char> = string.chars().collect();
            sorted_string.sort_unstable();
            anagrams.entry(sorted_string).or_default().push(string);
        }
        anagrams.values().cloned().collect()
    }
}
