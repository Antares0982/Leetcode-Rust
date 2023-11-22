const LT: &str = "&lt;";
const GT: &str = "&gt;";
const FRASL: &str = "&frasl;";
const AMP: &str = "&amp;";
const APOS: &str = "&apos;";
const QUOT: &str = "&quot;";

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut ret = String::new();
        let mut skip_index = 0;
        for (i, ch) in text.chars().enumerate() {
            if skip_index > 0 {
                skip_index = skip_index - 1;
                continue;
            }
            let mut new_ch: char = ch;
            if ch == '&' {
                new_ch = match &text[i..] {
                    slice if slice.len() >= LT.len() && &slice[..LT.len()] == LT => {
                        skip_index = LT.len() - 1;
                        '<'
                    }
                    slice if slice.len() >= GT.len() && &slice[..GT.len()] == GT => {
                        skip_index = GT.len() - 1;
                        '>'
                    }
                    slice if slice.len() >= FRASL.len() && &slice[..FRASL.len()] == FRASL => {
                        skip_index = FRASL.len() - 1;
                        '/'
                    }
                    slice if slice.len() >= AMP.len() && &slice[..AMP.len()] == AMP => {
                        skip_index = AMP.len() - 1;
                        '&'
                    }
                    slice if slice.len() >= APOS.len() && &slice[..APOS.len()] == APOS => {
                        skip_index = APOS.len() - 1;
                        '\''
                    }
                    slice if slice.len() >= QUOT.len() && &slice[..QUOT.len()] == QUOT => {
                        skip_index = QUOT.len() - 1;
                        '"'
                    }
                    _ => '&',
                };
            }
            ret.push(new_ch);
        }
        ret
    }
}
