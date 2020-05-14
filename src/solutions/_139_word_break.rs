pub struct Solution;
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut a = vec![false; n + 1];
        let hs: HashSet<String> = HashSet::from_iter(word_dict);
        a[0] = true;
        for i in 1..=n {
            for j in 0..i {
                if a[j] && hs.contains(&s[j..i]) {
                    a[i] = true;
                    break;
                }
            }
        }
        a[n]
    }
}

#[test]
fn test() {
    let s = "leetcode".to_string();
    let word_dict = vec_string!["leet", "code"];
    assert_eq!(Solution::word_break(s, word_dict), true);
    let s = "applepenapple".to_string();
    let word_dict = vec_string!["apple", "pen"];
    assert_eq!(Solution::word_break(s, word_dict), true);
    let s = "catsandog".to_string();
    let word_dict = vec_string!["cats", "dog", "sand", "and", "cat"];
    assert_eq!(Solution::word_break(s, word_dict), false);
}
