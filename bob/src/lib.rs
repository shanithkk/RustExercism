pub fn reply(message: &str) -> &str {

	let message=message.trim();
	let response :&str;
	if message.is_empty(){
		response="Fine. Be that way!";
	} 
	else if message.to_uppercase()==message&& message.ends_with("?") &&message.chars().any(char::is_alphabetic){
		response ="Calm down, I know what I'm doing!";
	}
	else if message.ends_with("?"){
		response="Sure.";
	}
	else if message.chars().any(char::is_alphabetic) &&message.to_uppercase()==message {
		response="Whoa, chill out!";		
	}
	else {
		response="Whatever.";
	}
response



}
