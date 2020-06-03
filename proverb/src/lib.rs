pub fn build_proverb(list: &[&str]) -> String {
   	let l=list.len()-1;
	let mut strings= String::new();
   	if list.is_empty(){
   		return strings
   		}
	
	for i in 0..l{
		
		strings += &(format!("For want of a {} the {} was lost.",list[i],list[i+1])+"\n" );

		}
	strings += &format!("And all for the want of a {}.",list[0]);

	strings
}
