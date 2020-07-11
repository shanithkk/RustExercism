use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    map: HashMap<&'a str, &'a str>,
}

const STOP_CODONS: [&'static str; 3] = ["UAA", "UAG", "UGA"];

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.map.get(codon).map(|&c| c)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let rna_len = rna.len();
        if rna_len < 3 {
            return None;
        }

        let mut proteins = vec![];
        let mut start = 0;
        while start < rna_len {
            let end = (start + 3).min(rna_len);
            let codon = &rna[start..end];
            if STOP_CODONS.contains(&codon) {
                break;
            }

            if let Some(&protein) = self.map.get(codon) {
                proteins.push(protein);
            } else {
                return None;
            }

            start = end;
        }

        if proteins.is_empty() {
            None
        } else {
            Some(proteins)
        }
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let mut map = HashMap::new();
    pairs.into_iter().for_each(|(codon, name)| {
        map.insert(codon, name);
    });
    CodonsInfo { map }
}