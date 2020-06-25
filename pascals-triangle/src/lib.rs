pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let row_count = row_count as usize;
        
        let mut rows = Vec::with_capacity(row_count);
        
        if row_count > 0 {
            rows.push(vec![0, 1, 0]);
            
            for i in 1..row_count {
                let mut new_row = Vec::with_capacity(i + 1);
                
                new_row.push(1);
                for j in 1..i  {
                    new_row.push(rows[i - 1][j - 1] + rows[i - 1][j])
                }
                new_row.push(1);
                
                rows.push(new_row)
            }
            
            rows[0] = vec![1];
        }
        
        Self { rows }
    }
    
    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}