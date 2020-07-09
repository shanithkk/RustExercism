pub fn is_pangram(sentence: &str) -> bool {
	let sentence=sentence.to_lowercase();
    let a = "abcdefghijklmnopqrstuvwxyz";
    let mut b=0;
    for i in a.chars(){
    	if sentence.contains(i){
    		b+=1;
    		continue;
    	}
    	else{
    		break;
    	}
    }
    b==26
}