#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for mut v in values.iter().cloned() {
        let mut bytes: Vec<u8> = Vec::new();
        if v == 0 {
            result.push(0);
            continue;
        }
        let mut start_byte = true;
        while v > 0 {
            if start_byte {
                bytes.push((v % 128) as u8);
                start_byte = false;
            } else {
                bytes.push((v % 128 | 0x80) as u8);
            }
            v >>= 7;
        }
        while let Some(x) = bytes.pop() {
            result.push(x)
        }
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.is_empty() || (bytes.last().unwrap() & 0x80u8 > 0) {
        return Err(Error::IncompleteNumber);
    }
    let mut result: Vec<u32> = Vec::new();
    let mut n = 0u32;
    for b in bytes {
        n = (n << 7) + (*b & 0x7f) as u32;
        println!("{}", n);
        if *b & 0x80u8 == 0 {
            result.push(n);
            n = 0;
            continue;
        }
        if n >= 1 << 25 {
            return Err(Error::Overflow);
        }
    }
    Ok(result)
}