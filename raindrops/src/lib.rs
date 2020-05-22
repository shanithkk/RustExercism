pub fn raindrops(n: u32) -> String {

    let mut word=String::from(""); 
    if n % 3== 0{
    	word.push_str("Pling");
    }
    if n%5 ==0{
    	word.push_str("Plang");
    }
    if n%7 ==0{
    	word.push_str("Plong");
    }
    if word==""{ 
    	word = n.to_string();
    }

	word
}
