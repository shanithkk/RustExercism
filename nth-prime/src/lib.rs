pub fn nth(n: u32) -> u32 {
    let mut prime_counter: u32 = 1;
    let mut current_number: u32 = 3;
    let mut prime_numbers = vec![2];

    while prime_counter <= n {
        let mut is_prime = true;
        for prime in &prime_numbers {
            if current_number % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            prime_numbers.push(current_number);
            prime_counter += 1;
        }

        current_number += 2;
    }
    prime_numbers.pop().unwrap()
}