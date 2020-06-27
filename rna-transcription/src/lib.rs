#[derive(Debug, PartialEq)]
pub struct DNA(String);

#[derive(Debug, PartialEq)]
pub struct RNA(String);

impl DNA {
    pub fn new(dna: &str) -> Result<DNA, usize> {
        let mut result = String::new();
        for (i, c) in dna.char_indices() {
            match c {
                'A' | 'C' | 'G' | 'T' => result.push(c),
                _ => return Err(i)
            }
        }
        Ok(DNA(result))
    }

    pub fn into_rna(self) -> RNA {
        RNA(self.0.chars().map(|c| {
            match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!("DNA has illegal value: {}", c),
            }
        }).collect())
    }
}

impl RNA {
    pub fn new(rna: &str) -> Result<RNA, usize> {
        let mut result = String::new();
        for (i, c) in rna.char_indices() {
            match c {
                'A' | 'C' | 'G' | 'U' => result.push(c),
                _ => return Err(i)
            }
        }
        Ok(RNA(result))
    }
}