pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group_hm: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut v: Vec<char> = s.chars().collect();
            v.sort_unstable();
            let group = group_hm.entry(v).or_default();
            group.push(s);
        }
        let mut res: Vec<Vec<String>> = vec![];
        for (_, mut v) in group_hm {
            v.sort();
            res.push(v.to_vec());
        }
        res.sort_by(|a, b| a.len().cmp(&b.len()));
        res
    }
}

#[test]
fn test() {
    let strs: Vec<String> = vec_string!["eat", "tea", "tan", "ate", "nat", "bat"];
    let res: Vec<Vec<String>> = vec_vec_string![["bat"], ["nat", "tan"], ["ate", "eat", "tea"]];
    assert_eq!(Solution::group_anagrams(strs), res);
}
