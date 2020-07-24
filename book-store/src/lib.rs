pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }

    let mut sets: Vec<Vec<u32>> = Vec::new();
    for &book in books {
        let mut added_book_to_set = false;
        for set in sets.iter_mut() {
            if !set.contains(&book) {
                set.push(book);
                added_book_to_set = true;
                break;
            }
        }
        if !added_book_to_set {
            sets.push(vec![book])
        }
        if sets[0].len() != 3 {
            sets.rotate_left(1);
        }
    }

    sets.iter().fold(0, |acc, set| {
        acc + match set.len() {
            0 => 0,
            1 => 800,
            2 => 1520,
            3 => 2160,
            4 => 2560,
            5 => 3000,
            _ => unreachable!(),
        }
    })
}