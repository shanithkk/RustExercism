use std::collections::HashMap;

fn is_nucleotide(n: char) -> bool {
    match n {
        'A' | 'G' | 'T' | 'C' => true,
        _ => false
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {

    if !is_nucleotide(nucleotide) {
        return Err(nucleotide)
    }

    let mut res: usize = 0;
    for c in dna.chars() {
        match c {
            _ if c == nucleotide => {
                res += 1;
            },
            'A' | 'G' | 'T' | 'C' => continue,
            _ => return Err(c)
        }
    }

    Ok(res)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {

    let mut count_map: HashMap<char, usize> = HashMap::new();

    count_map.insert('A', 0);
    count_map.insert('G', 0);
    count_map.insert('T', 0);
    count_map.insert('C', 0);

    for c in dna.chars() {

        match count_map.get(&c) {
            Some(&count) => {
                count_map.insert(c, count + 1);
            },
            None => return Err(c)
        }

    }

    Ok(count_map)
}