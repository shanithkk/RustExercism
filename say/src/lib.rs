pub fn encode(n: u64) -> String {
    if n == 0 { return "zero".to_owned(); }
    let mut number = n;
    let mut final_string = "".to_string();
    let thousands_names = ["thousand", "million", "billion", "trillion", "quadrillion", "quintillion"];

    let mut thousands_count = 0;
    while number > 0 {
        let last_3_digits = number % 1000;

        let mut end = hundreds(last_3_digits).trim().to_string();
        if thousands_count > 0 && last_3_digits != 0 {
            end += format!(" {} ", thousands_names[thousands_count as usize - 1]).as_ref();
        }

        final_string = format!("{}{}", end, final_string);
        thousands_count += 1;
        number /= 1000;
    }

    final_string.trim().to_string()
}

fn hundreds(n: u64) -> String {
    let huns = (n / 100) % 10;
    if huns == 0 {
        tens(n % 100)
    } else {
        ones(huns) + " hundred " + tens(n % 100).as_ref()
    }
}

fn tens(n: u64) -> String {
    let units = n % 10;
    let tenth = (n / 10) % 10;
    let last_two_digits = n % 100;

    let less_20 = ["ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", ];
    let more_20 = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety", ];

    match last_two_digits {
        0..=9 => ones(last_two_digits),
        10..=19 => less_20[last_two_digits as usize - 10].to_string(),
        _ => {
            let mut number = more_20[tenth as usize - 2].to_string();
            if units != 0 {
                number += "-";
                number += ones(units).as_ref();
            }
            number
        }
    }
}

fn ones(n: u64) -> String {
    ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"][n as usize].to_string()
}