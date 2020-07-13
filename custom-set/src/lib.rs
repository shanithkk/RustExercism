#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    data: Vec<T>,
}

impl<T: Clone + Ord + PartialEq> CustomSet<T> {
    pub fn new(input: &[T]) -> Self {
        let mut data: Vec<T> = input.into();
        data.sort_unstable();
        Self { data: data }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.binary_search(element).is_ok()
    }

    pub fn add(&mut self, element: T) {
        match self.data.binary_search(&element) {
            Ok(_) => (),
            Err(index) => self.data.insert(index, element),
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.difference(other).is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.intersection(other).is_empty()
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut result_data = Vec::new();
        for element in &self.data {
            if other.contains(element) {
                result_data.push(element.clone());
            }
        }

        Self { data: result_data }
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut result_data = Vec::new();
        for element in &self.data {
            if !other.contains(element) {
                result_data.push(element.clone());
            }
        }

        Self { data: result_data }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut result_data = other.data.clone();
        for element in &self.data {
            if !other.contains(element) {
                result_data.push(element.clone());
            }
        }
        result_data.sort_unstable();

        Self { data: result_data }
    }
}