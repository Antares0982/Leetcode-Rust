impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut ret = 0;
        while i < j {
            while nums[i] + nums[j] >= target && j > i {
                j -= 1;
            }
            if j <= i {
                break;
            }
            // counting [i+1..j], is j-i
            ret += j - i;
            i += 1;
        }
        ret as i32
    }
}
