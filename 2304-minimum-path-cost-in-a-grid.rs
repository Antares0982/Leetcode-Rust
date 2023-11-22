use std::cmp;

impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut dp0 = grid[0].clone();

        for x in 1..m {
            let mut dp1 = vec![i32::MAX; n];
            for i in 0..n {
                let grid_val = grid[x][i];
                for (j, &value) in dp0.iter().enumerate() {
                    let last_grid_val = grid[x - 1][j];
                    let cost = move_cost[last_grid_val as usize][i];
                    dp1[i] = cmp::min(dp1[i], cost + value + grid_val);
                }
            }
            dp0 = dp1;
        }
        let ret = dp0.iter().min();

        match ret {
            Some(value) => return *value,
            _ => panic!("n==0"),
        }
    }
}
