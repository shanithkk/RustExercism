use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut results: HashSet<[u32; 3]> = HashSet::new();
    for side1 in 1..=sum / 3 {
        for side2 in side1 + 1..sum / 2 {
            
            let side3 = sum - side1 - side2;
            if side1 * side1 + side2 * side2 == side3 * side3 {
                
                results.insert([side1, side2, side3]);
            }
        }
    }
    results
}