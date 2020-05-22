pub fn is_leap_year(year: u64) -> bool {
    
	let mut is_leap = true;
	if year%4 ==0 {
		if year%100==0 {
			if year%400==0 {
				println!("{} is a leap year", year );
			} else {
					is_leap = false;
				println!("{} is not a leap year", year );
			}
		}else {
			println!("{}is a leap year", year );
		}
	}else {
		is_leap=false;
		println!("{} is not a leap year", year );
		}
		is_leap
}
