pub struct Solution;
use util::*;

trait Inorder {
    fn inorder(&self, visit: &mut dyn FnMut(i32));
}

impl Inorder for TreeLink {
    fn inorder(&self, visit: &mut dyn FnMut(i32)) {
        if let Some(node) = self {
            let node = node.borrow();
            Self::inorder(&node.left, visit);
            visit(node.val);
            Self::inorder(&node.right, visit);
        }
    }
}

impl Solution {
    pub fn is_valid_bst(root: TreeLink) -> bool {
        let mut prev: Option<i32> = None;
        let mut res = true;
        root.inorder(&mut |x| {
            if let Some(y) = prev {
                if x <= y {
                    res = false;
                }
            }
            prev = Some(x);
        });
        res
    }
}

#[test]
fn test() {
    let root = tree!(2, tree!(1), tree!(3));
    assert_eq!(Solution::is_valid_bst(root), true);
    let root = tree!(5, tree!(1), tree!(4, tree!(3), tree!(6)));
    assert_eq!(Solution::is_valid_bst(root), false);
}
