use primal;
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
	
	if upper_bound<=1{
    	return vec![]
    }
	let  a : Vec<u64> = (2..=upper_bound).collect();
	let mut b :Vec<u64>=vec![];
	
    for i in a{
       	if primal::is_prime(i){
       		b.push(i);
       	}
    }
  b
}
