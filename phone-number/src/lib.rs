pub fn number(user_number: &str) -> Option<String> {
	
	let s:String = user_number.chars()
    .map(|x| match x { 
        '('|')'|' '|'.'|'-'|'+' => '!', 
        _ => x
    }).collect();
    let mut result = str::replace(s.as_str(), "!", "");
    if result.chars().nth(0) ==Some('1') {
    	 result.remove(0);
    		
    }
    let num :Vec<char>=result.chars().collect();
    if num.len()==10{
    	if (num[0]>='2'&&num[0]<='9')&&(num[3] >= '2' && num[3] <= '9'){
    		return Some(result);
    	}
    }
   
None
    
}