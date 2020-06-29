pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size];
    if size == 0 {
        return matrix;
    }

    matrix[0][0] = 1;

    let (mut i, mut j) = (0usize, 0usize);
    let mut val = 2u32;
    let val_max = (size * size) as u32;
    while val <= val_max {
        // right
        while j <= size - 2 && matrix[i][j + 1] == 0 {
            j += 1;
            matrix[i][j] = val;
            val += 1;
        }

        // down
        while i <= size - 2 && matrix[i + 1][j] == 0 {
            i += 1;
            matrix[i][j] = val;
            val += 1;
        }

        // left
        while j >= 1 && matrix[i][j - 1] == 0 {
            j -= 1;
            matrix[i][j] = val;
            val += 1;
        }

        // up
        while i >= 1 && matrix[i - 1][j] == 0 {
            i -= 1;
            matrix[i][j] = val;
            val += 1;
        }
    }
    matrix
}