use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for (&score, letters) in h.iter() {
        for &letter in letters.iter() {
            result.insert(letter.to_lowercase().next().unwrap(), score);
        }
    }
    result
}