pub fn brackets_are_balanced(string: &str) -> bool {
    let mut b = Vec::new();
    for c in string.chars() 
    {
        if c =='{' || c=='[' || c=='(' 
        {
        	b.push(c);
        	
        }
        else if c =='}' 
        {
        	if b.pop()!= Some('{'){
        		
        		return false
        	}
        } 
        else if c ==']' 
        {
        	if b.pop()!= Some('['){
        		
        		return false
        	}
        }
        else if c ==')' 
        {
        	if b.pop()!= Some('('){
        		
        		return false
        	}
        }   

    
    }
    b.is_empty()
}