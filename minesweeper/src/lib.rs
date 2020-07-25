use std::collections::HashSet;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mine_table = make_mine_table(minefield);
    let mut result = Vec::new();

    let mut i = 0;
    for row in minefield {
        let mut j = 0;
        let mut result_row = String::new();
        for c in row.chars() {
            result_row.push(match c {
                '*' => '*',
                _ => compute_mines(&mine_table, i, j),
            });
            j += 1;
        }
        result.push(result_row);
        i += 1;
    }

    return result;
}

fn make_mine_table(minefield: &[&str]) -> HashSet<(i32, i32)> {
    let mut result = HashSet::new();
    let mut i = 0;
    for row in minefield {
        let mut j = 0;
        for c in row.chars() {
            if '*' == c {
                result.insert((i, j));
            }
            j += 1;
        }
        i += 1;
    }
    return result;
}

fn compute_mines(mine_table: &HashSet<(i32, i32)>, i: i32, j: i32) -> char {
    let count = neighbors(i, j)
        .iter()
        .filter_map(|x| mine_table.get(x))
        .count();
    if 0 == count {
        ' '
    } else {
        (b'0' + count as u8) as char
    }
}

fn neighbors(x: i32, y: i32) -> [(i32, i32); 8] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}