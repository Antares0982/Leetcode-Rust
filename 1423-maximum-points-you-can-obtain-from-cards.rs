use std::cmp::max;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut ret = 0;
        let sz = card_points.len();
        for i in 0..(k as usize) {
            ret += card_points[i];
        }
        if k as usize == sz {
            return ret;
        }
        let mut cur = ret;
        
        for i in 0..k {
            cur += card_points[sz - 1 - i as usize];
            cur -= card_points[(k - 1 - i) as usize];
            ret = max(cur, ret);
        }

        ret
    }
}
