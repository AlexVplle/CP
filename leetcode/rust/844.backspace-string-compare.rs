// @leet start
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();
        let (mut s_pointer, mut t_pointer): (i32, i32) = (s.len() as i32 - 1, t.len() as i32 - 1);
        let (mut s_skip_count, mut t_skip_count): (usize, usize) = (0, 0);
        while s_pointer >= 0 || t_pointer >= 0 {
            while s_pointer >= 0 {
                if s[s_pointer as usize] == '#' {
                    s_skip_count += 1;
                    s_pointer -= 1;
                } else if s_skip_count > 0 {
                    s_skip_count -= 1;
                    s_pointer -= 1;
                } else {
                    break;
                }
            }
            while t_pointer >= 0 {
                if t[t_pointer as usize] == '#' {
                    t_skip_count += 1;
                    t_pointer -= 1;
                } else if t_skip_count > 0 {
                    t_skip_count -= 1;
                    t_pointer -= 1;
                } else {
                    break;
                }
            }
            if s_pointer >= 0 && t_pointer >= 0 {
                if s[s_pointer as usize] != t[t_pointer as usize] {
                    return false;
                }
            } else {
                if s_pointer >= 0 || t_pointer >= 0 {
                    return false;
                }
            }
            s_pointer -= 1;
            t_pointer -= 1;
        }
        true
    }
}
// @leet end
