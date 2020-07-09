 pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut digits=digits.to_string();
    let mut a =vec![];
    if len==0{
    	return vec!["".to_string(); digits.len() + 1];
    }
    if len>digits.len(){
    	return a;
    }
    let b=digits.len()-len;
    for _i in 0..=b{
    	let c=&digits[0..len];
    	a.push(c.to_string());
    	digits.remove(0);
    }
a
}