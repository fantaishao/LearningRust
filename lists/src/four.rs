impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() {
            return false
        }

        let t:Vec<Vec<char>> = [['(', ')'], ['{', '}'], ['[', ']']];
        let w: Vec<char> = s.chars().collect();
        let result = true;
        for (right, &ch) in t.inter.emunerate() {
            let n = s.indexOf(&ch[0]);
            let m = s.indexOf(&ch[1]);
            if (m != - 1 && n == -1) || (m == -1 && n != -1) {
                result = false;
                break;
            } else if (m - n) % 2 != 0 {
                result = false;
                break;
            }

            result = true
        }
        result
    }
}