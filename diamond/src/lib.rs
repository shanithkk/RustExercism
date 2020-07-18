pub fn get_diamond(c: char) -> Vec<String> {
    let mut grid = vec![];
    let leg = ((c as u8) - b'A') as i32;
    let size = 2 * leg + 1;

    for row in 0..size {
        let mut line = String::new();
        let left_offset;
        let letter;

 
        if row <= leg {
            left_offset = leg - row;
            letter = (b'A' + row as u8) as char;
        } else {
            left_offset = row - leg;
            letter = (b'A' + leg as u8 - left_offset as u8) as char;
        };
        let right_offset = size - left_offset - 1;

 
        for col in 0..size {
            if col == left_offset || col == right_offset {
                line.push(letter);
            } else {
                line.push(' ');
            }
        }
        grid.push(line);
    }

    grid
}