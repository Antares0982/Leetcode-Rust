use std::cmp::max;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut ret = i32::MIN;
        let mut sum = 0;
        for v in nums.iter() {
            sum += v;
            ret = max(ret, sum);
            if sum < 0 {
                sum = 0;
            }
        }
        ret
    }
}
