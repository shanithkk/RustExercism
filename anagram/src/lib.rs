use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let orig = to_ordered_string(word);
    possible_anagrams
        .iter()
        .filter_map(|w| {
            let sample = to_ordered_string(w);
            if sample == orig && w.to_lowercase() != word.to_lowercase() {
                Some(w)
            } else {
                None
            }
        })
        .cloned()
        .collect()
}

fn to_ordered_string(word: &str) -> String {
    let mut orig = word.to_lowercase().chars().collect::<Vec<char>>();
    orig.sort();
    orig.iter().collect::<String>()
}