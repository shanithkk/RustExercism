pub fn is_valid_isbn(isbn: &str) -> bool {
    
	let input = isbn.replace("-", "");

    let for_fact = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let for_length = for_fact.len();
    if input.len() != for_length {
        return false;
    }
    let mut vec =vec![];
    let mut vec1 =vec![];
    for i in  input.chars(){
    	vec.push(i);
    }

    for i in 0..=9{
    	
    	if vec[i]=='0'||vec[i]=='1'||vec[i]=='2'||vec[i]=='3'||vec[i]=='4'||vec[i]=='5'||vec[i]=='6'||vec[i]=='7'||vec[i]=='8'||vec[i]=='9'{
			let j= (vec[i].to_string()).parse::<i32>().unwrap(); 
    		vec1.push(j);
    	}
    	else if vec[9]=='X'||vec[9]=='x'{
    		vec1.push(10);
    	}
    	else {
    		return false
    	}
    }
    
    let mut sum =0;
    for i in 0..10{
    	sum += vec1[i]*for_fact[i];
    }
    
    sum%11==0
    
}