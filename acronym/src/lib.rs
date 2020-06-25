pub fn abbreviate(phrase: &str) -> String {
    let mut acronym = String::from("");
    if phrase.is_empty() { 
        return acronym; 
    }
    else {
        for s in phrase.split(" ") {
            if s.is_empty() == false {
                let mut firstchar: bool = false;
                let mut is_prev_char_lowercase: bool = false;
                for c in s.chars() {
                    if !firstchar {
                        if c != '_' && 
                            c != '-'
                        {
                            acronym.push(c.to_uppercase().next().unwrap());
                            firstchar = true;
                        }
                    }
                    else if c == '-' {
                        firstchar = false;
                    }
                    else if c.is_uppercase() && is_prev_char_lowercase {
                        acronym.push(c); 
                    }
                    if c.is_lowercase() {
                        is_prev_char_lowercase = true;
                    }
                }
            }
        }
    }
    acronym
}