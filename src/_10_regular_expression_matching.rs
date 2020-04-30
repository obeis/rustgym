struct Solution;

impl Solution {
    fn is_match(s: String, p: String) -> bool {
        let n = s.len();
        let m = p.len();
        let s = s.chars().collect();
        let p = p.chars().collect();
        let mut memo: Vec<Vec<Option<bool>>> = vec![vec![None; m + 1]; n + 1];
        Self::is_match_dp(n, m, &mut memo, &s, &p)
    }

    fn is_match_dp(
        n: usize,
        m: usize,
        memo: &mut Vec<Vec<Option<bool>>>,
        s: &Vec<char>,
        p: &Vec<char>,
    ) -> bool {
        if let Some(ans) = memo[n][m] {
            ans
        } else {
            let res = if n == 0 && m == 0 {
                true
            } else if n != 0 && m == 0 {
                false
            } else if n == 0 && m != 0 {
                if p[m - 1] == '*' {
                    Self::is_match_dp(n, m - 2, memo, s, p)
                } else {
                    false
                }
            } else {
                if s[n - 1] == p[m - 1] {
                    Self::is_match_dp(n - 1, m - 1, memo, s, p)
                } else {
                    match p[m - 1] {
                        '*' => match p[m - 2] {
                            '*' => false,
                            '.' => {
                                Self::is_match_dp(n - 1, m, memo, s, p)
                                    || Self::is_match_dp(n, m - 2, memo, s, p)
                            }
                            _ => {
                                if s[n - 1] != p[m - 2] {
                                    Self::is_match_dp(n, m - 2, memo, s, p)
                                } else {
                                    Self::is_match_dp(n - 1, m, memo, s, p)
                                        || Self::is_match_dp(n, m - 2, memo, s, p)
                                }
                            }
                        },
                        '.' => Self::is_match_dp(n - 1, m - 1, memo, s, p),
                        _ => false,
                    }
                }
            };

            memo[n][m] = Some(res);
            res
        }
    }
}

#[test]
fn test() {
    let s = "aa".to_string();
    let p = "a".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "aa".to_string();
    let p = "a*".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "ab".to_string();
    let p = ".*".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "aab".to_string();
    let p = "c*a*b".to_string();
    let res = true;
    assert_eq!(Solution::is_match(s, p), res);
    let s = "mississippi".to_string();
    let p = "mis*is*p*.".to_string();
    let res = false;
    assert_eq!(Solution::is_match(s, p), res);
}
