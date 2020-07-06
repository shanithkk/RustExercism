#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let mut sum = 0_u64;
    if num==0{
    	return None;
    }
    for i in 1..num {
        if num % i == 0 {
            sum += i;
        }
    }
    if sum == num {
        Some(Classification::Perfect)
    } else if sum > num {
        Some(Classification::Abundant)
    } else {
        Some(Classification::Deficient)
    }
}