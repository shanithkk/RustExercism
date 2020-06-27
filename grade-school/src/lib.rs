use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
pub struct School {
    data: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.data.entry(grade).or_insert_with(BTreeSet::new).insert(student.to_owned());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.data.keys().cloned().collect()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.data.get(&grade).map(|x| x.iter().cloned().collect())
    }
}