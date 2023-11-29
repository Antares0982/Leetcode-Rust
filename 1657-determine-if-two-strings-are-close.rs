impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() { return false; }
        
        let mut word1_count = [0; 26];
        let mut word2_count = [0; 26];

        for (c1, c2) in word1.chars().zip(word2.chars()) {
            word1_count[c1 as usize - 'a' as usize] += 1;
            word2_count[c2 as usize - 'a' as usize] += 1;
        }

        if !word1_count.iter().zip(&word2_count).all(|(a, b)| (*a == 0) == (*b == 0)) {
            return false;
        }

        word1_count.sort_unstable();
        word2_count.sort_unstable();

        word1_count == word2_count
    }
}
