use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: Vec<u64>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome {
            value: a * b,
            factors: vec![std::cmp::min(a, b)]
        }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn insert(&mut self, a: u64, b: u64) {
        self.factors.push(std::cmp::min(a, b))
    }
}

pub fn reverse_digits(v: u64) -> u64 {
    let mut s = v;
    let mut r = 0;
    while s != 0 {
        r = 10 * r + s % 10;
        s /= 10;
    }
    r
}

fn is_palindrome(v: u64) -> bool {
    v == reverse_digits(v)
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut result: Option<(Palindrome, Palindrome)> = None;

    for a in min..=max {
        for b in a..=max {
            let v = a * b;
            if is_palindrome(v) {
                match result {
                    Some(ref mut pals) => {
                        match v.cmp(&pals.0.value()) {
                            Ordering::Less => pals.0 = Palindrome::new(a, b),
                            Ordering::Equal => pals.0.insert(a, b),
                            Ordering::Greater => {}
                        }
                        match v.cmp(&pals.1.value()) {
                            Ordering::Greater => pals.1 = Palindrome::new(a, b),
                            Ordering::Equal => pals.1.insert(a, b),
                            Ordering::Less => {}
                        }
                    }
                    None => result = Some((Palindrome::new(a, b), Palindrome::new(a, b)))
                }
            }
        }
    }

    result
}