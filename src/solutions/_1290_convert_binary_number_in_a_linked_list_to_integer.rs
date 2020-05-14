pub struct Solution;
use util::*;

impl Solution {
    pub fn get_decimal_value(head: ListLink) -> i32 {
        let mut p: &ListLink = &head;
        let mut res = 0;
        while let Some(n) = p {
            res *= 2;
            res += n.val;
            p = &n.next;
        }
        res
    }
}

#[test]
fn test() {
    let head = list!(1, 0, 1);
    assert_eq!(Solution::get_decimal_value(head), 5);
}
