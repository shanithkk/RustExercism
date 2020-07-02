pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = Vec::new();

    for (row_i, row) in input.iter().enumerate() {
        let mut row_max_val = 0;
        for &col_val in row {
            if col_val > row_max_val {
                row_max_val = col_val;
            }
        }

        for (col_i, &col_val) in row.iter().enumerate() {
            if col_val == row_max_val {
                let mut minimum = true;

                for row_b in input {
                    if *row_b.get(col_i).unwrap() < row_max_val {
                        minimum = false;
                        break;
                    }
                }

                if minimum {
                    output.push((row_i, col_i));
                }
            }
        }
    }
    output
}