
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }
    let mut distance = 0usize;
    for i in 0..s1.len() {
        if s1[i..i+1] != s2[i..i+1] {

            distance += 1;
        }
      } 
    Some(distance)
}

