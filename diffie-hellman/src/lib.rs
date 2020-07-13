use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    modular_pow(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    modular_pow(b_pub, a, p)
}

fn modular_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    if base == 0 {
        return 0;
    }
    
    let mut base = (base % modulus) as u128;
    let modulus = modulus as u128;
    
    let mut exp = exp;
    
    let mut res = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % modulus;
        }
        
        exp >>= 1;
        base = base * base % modulus;
    }
    
    res as u64
}