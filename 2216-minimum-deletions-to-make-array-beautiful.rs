use std::cmp::max;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        // dp1[i] is nums[:i] ans
        let mut dp1: Vec<i32> = Vec::new();
        dp1.push(0);
        let mut last_different_index: i32 = -1;
        for (i, v) in nums.iter().enumerate() {
            let mut mark_index: i32 = i as i32;
            if i > 0 {
                if nums[i - 1] != *v {
                    mark_index = i as i32 - 1;
                } else {
                    if last_different_index >= 0 {
                        mark_index = last_different_index;
                    }
                }
            }
            let mut append = 0;
            if mark_index >= 0 && mark_index < i as i32 {
                // choose from nums[:mark_index]
                append = dp1[mark_index as usize] + 2;
            }
            dp1.push(max(append, dp1[dp1.len() - 1]));

            if i > 0 && nums[i - 1] != *v {
                last_different_index = i as i32 - 1;
            }
        }
        let mut ret = 0;
        if dp1.len() > 0 {
            ret = dp1[dp1.len() - 1];
        }
        nums.len() as i32 - ret
    }
}
