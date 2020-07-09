#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    }

    if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    }

    let mut value = 0;
    for (i, digit) in number.iter().rev().enumerate() {
        if digit >= &from_base {
            return Err(Error::InvalidDigit(*digit));
        }
        value += digit * from_base.pow(i as u32)
    }

    let mut converted = vec![];
    while value > 0 {
        converted.push(value % to_base);
        value /= to_base;
    }
    converted.reverse();

    if converted.is_empty() {
        converted.push(0);
    }

    Ok(converted)
}