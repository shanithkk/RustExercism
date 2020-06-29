pub fn check(candidate: &str) -> bool {
    let a =candidate.to_lowercase();
    let mut b =vec![];
    for i in a.chars(){ 
    	if i==' '||i=='-'||i=='_'{
    		continue;
    	}
    	else{
    	 	b.push(i);
    	}
    }
    let mut c=0;
    for i in &b{
    	for j in &b{
    		if i==j{c+=1;}
    	}
    	if c>=2{
    		return false
       	}
    	c=0;
    }
    true
}

