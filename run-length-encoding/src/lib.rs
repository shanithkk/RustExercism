pub fn encode(source: &str) -> String {
    let mut s = String::new();
    let mut count = 0;
    let mut cur_ch: char = '#';
    for ch in source.chars() {
        if cur_ch == ch {
            count += 1;
            continue;
        }
        if count == 0 {
            cur_ch = ch;
            count += 1;
            continue;
        }
        if count > 1 {
            s = format!("{}{}", s, count);
        }
        s.push(cur_ch);
        cur_ch = ch;
        count = 1;
    }
    if count > 1 {
        s = format!("{}{}", s, count);
    }
    s.push(cur_ch);
    s
}

pub fn decode(source: &str) -> String {
    let mut s = String::new();
    let mut count: usize;
    let mut count_str = String::new();
    for ch in source.chars() {
        if ch.is_digit(10) {
            count_str.push(ch);
        }
        else {
            count = count_str.parse().unwrap_or(1);
            for _ in 0..count {
                s.push(ch);
            }
            count_str = String::new();
        }
    }
    s
}