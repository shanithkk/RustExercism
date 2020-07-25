use regex::Regex;

pub fn answer(command: &str) -> Option<i32> {
    let reg = Regex::new(r" ?(?P<oper>.*?) (?P<num>-?\d+)").unwrap();
    let (mut sum, mut total_len) = (0i32, 0usize);
    for cap in reg.captures_iter(command) {
        let num = cap["num"].parse::<i32>().unwrap();
        total_len += cap[0].len(); // Add the length of the treated match
        match &cap["oper"] {
            "What is" => sum = num,
            "plus" => sum += num,
            "minus" => sum -= num,
            "multiplied by" => sum *= num,
            "divided by" => sum /= num,
            _ =>return None,
        }
    }
    // Checks if all the command string has been captured
    if total_len + 1 /* last ? */ != command.len() {
        None
    } else {
        Some(sum)
    }
}