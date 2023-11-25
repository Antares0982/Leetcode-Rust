impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let mut ret = 0;
        let mut marks: [[i32; 2]; 26] = [[-1; 2]; 26];
        let mut last_diff = 0;
        for (index, c) in s.chars().enumerate() {
            let mark_index = (c as u8 - 'A' as u8) as usize;
            let last_two_occur_index = marks[mark_index][0];
            let last_occur_index = marks[mark_index][1];
            // diff = sum [x, index+1) for x in 0..index+1, i.e. [0, index]
            // diff = sum [x, index+1) for x in [0, last_two_occur_index]
            //      + sum [x, index+1) for x in [last_two_occur_index+1, last_occur_index]
            //      + sum [x, index+1) for x in [last_occur_index+1, index]
            // diff = sum [x, index) for x in [0, last_two_occur_index]
            //      + sum [x, index) for x in [last_two_occur_index+1, last_occur_index] - (last_occur_index - last_two_occur_index)
            //      + sum [x, index) for x in [last_occur_index+1, index] + (index - last_occur_index)
            // diff = sum [x, index) for x in [0, index] + (index - last_occur_index) - (last_occur_index - last_two_occur_index)
            let new_diff = last_diff + index as i32 - 2 * last_occur_index + last_two_occur_index;
            ret += new_diff;
            // update last_diff
            last_diff = new_diff;
            // update marks
            marks[mark_index][0] = last_occur_index;
            marks[mark_index][1] = index as i32;
        }
        ret
    }
}
