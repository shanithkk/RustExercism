pub fn square_of_sum(n: u32) -> u32 {
    let mut res=0;
    for i in 1..=n{
    	res=res+i;	
    }
    let a= res*res;
    a
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut res=0;
    for i in 1..=n{
    	res= res+(i*i);
    }
    res
}

pub fn difference(n: u32) -> u32 {
    
    let a =square_of_sum(n);
	
	let b = sum_of_squares(n);

	let res=a-b;
	res
	
}
