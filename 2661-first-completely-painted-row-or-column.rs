impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut rows_map = vec![0; m * n];
        let mut cols_map = vec![0; m * n];
        for (i, row) in mat.iter().enumerate() {
            for (j, val) in row.iter().enumerate() {
                let ind = (*val - 1) as usize;
                rows_map[ind] = i;
                cols_map[ind] = j;
            }
        }
        let mut counts_row = vec![n; m];
        let mut counts_col = vec![m; n];
        for (index, &val) in arr.iter().enumerate() {
            let row = rows_map[val as usize - 1];
            let col = cols_map[val as usize - 1];
            if counts_row[row] == 1 {
                return index as i32;
            }
            counts_row[row] -= 1;
            if counts_col[col] == 1 {
                return index as i32;
            }
            counts_col[col] -= 1;
        }
        0
    }
}
