pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn shortest_word_distance(words: Vec<String>, word1: String, word2: String) -> i32 {
        let mut hm: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, word) in words.into_iter().enumerate() {
            hm.entry(word).or_default().push(i);
        }
        let mut min = std::usize::MAX;
        if word1 == word2 {
            let indexes = &hm[&word1];
            let n = indexes.len();
            for i in 1..n {
                min = min.min(indexes[i] - indexes[i - 1]);
            }
        } else {
            let indexes1 = &hm[&word1];
            let indexes2 = &hm[&word2];
            let n1 = indexes1.len();
            let n2 = indexes2.len();
            let mut i1 = 0;
            let mut i2 = 0;
            while i1 < n1 && i2 < n2 {
                if indexes1[i1] < indexes2[i2] {
                    min = min.min(indexes2[i2] - indexes1[i1]);
                    i1 += 1;
                } else {
                    min = min.min(indexes1[i1] - indexes2[i2]);
                    i2 += 1;
                }
            }
        }
        min as i32
    }
}

#[test]
fn test() {
    let words = vec_string!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "makes".to_string();
    let word2 = "coding".to_string();
    let res = 1;
    assert_eq!(Solution::shortest_word_distance(words, word1, word2), res);
    let words = vec_string!["practice", "makes", "perfect", "coding", "makes"];
    let word1 = "makes".to_string();
    let word2 = "makes".to_string();
    let res = 3;
    assert_eq!(Solution::shortest_word_distance(words, word1, word2), res);
}
