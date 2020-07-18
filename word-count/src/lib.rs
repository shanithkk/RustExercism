use std::collections::HashMap;

fn add_word(word: &str, count_by_word: &mut HashMap<String, u32>) {
    let wsl = word.to_string().to_lowercase();
    let counter = count_by_word.entry(wsl).or_insert(0);
    *counter += 1;
}

pub fn word_count(words: &str) -> HashMap<String, u32> {
    let single_quote = '\'';
    let mut start_word: usize = 0;
    let mut end_word: usize = 0;
    let mut count_by_word = HashMap::new();
    let mut prev_char = ' ';
    for c in words.chars() {
        if c.is_digit(36) {
            end_word += 1;
        } else if c == single_quote {
            if prev_char != single_quote {
                if end_word - start_word > 0 {
                    end_word += 1; 
                                   
                } else {
                    
                    start_word += 1;
                    end_word = start_word;
                }
            } else {
                
                start_word = end_word + 1;
                end_word = start_word;
            }
        } else {
            
            if end_word - start_word > 0 {
                let mut ew = end_word;
                if prev_char == single_quote {
                    ew -= 1; 
                }
                add_word(&words[start_word..ew], &mut count_by_word);
                start_word = end_word + 1;
                end_word = start_word;
            } else {
                start_word += 1;
                end_word = start_word;
            }
        }
        prev_char = c;
    }
    if end_word - start_word > 0 {
        
        add_word(&words[start_word..end_word], &mut count_by_word);
    }
    count_by_word
   }