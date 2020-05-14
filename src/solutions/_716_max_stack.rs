use std::i32;

#[derive(Default)]
pub struct MaxStack {
    pub vec: Vec<i32>,
}

impl MaxStack {
    pub fn new() -> Self {
        MaxStack { vec: vec![] }
    }

    pub fn push(&mut self, x: i32) {
        self.vec.push(x);
    }

    pub fn pop(&mut self) -> i32 {
        self.vec.pop().unwrap()
    }

    pub fn top(&self) -> i32 {
        *self.vec.last().unwrap()
    }

    pub fn peek_max(&self) -> i32 {
        let mut max = self.top();
        for &x in &self.vec {
            max = i32::max(max, x);
        }
        max
    }

    pub fn pop_max(&mut self) -> i32 {
        let mut index = self.vec.len() - 1;
        let mut max = self.top();
        let n = self.vec.len();
        for i in 0..n {
            let j = n - 1 - i;
            let x = self.vec[j];
            if x > max {
                max = x;
                index = j;
            }
        }
        self.vec.remove(index);
        max
    }
}

#[test]
fn test() {
    let mut stack = MaxStack::new();
    stack.push(5);
    stack.push(1);
    stack.push(5);
    assert_eq!(stack.top(), 5);
    assert_eq!(stack.pop_max(), 5);
    assert_eq!(stack.top(), 1);
    assert_eq!(stack.peek_max(), 5);
    assert_eq!(stack.pop(), 1);
    assert_eq!(stack.top(), 5);
}
