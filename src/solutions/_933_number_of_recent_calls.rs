use std::collections::VecDeque;

#[derive(Default)]
pub struct RecentCounter {
    pub queue: VecDeque<i32>,
}

impl RecentCounter {
    pub fn new() -> Self {
        RecentCounter {
            queue: VecDeque::new(),
        }
    }
    pub fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_back(t);
        while let Some(p) = self.queue.pop_front() {
            if p >= t - 3000 {
                self.queue.push_front(p);
                break;
            }
        }
        self.queue.len() as i32
    }
}

#[test]
pub fn name() {
    let mut obj = RecentCounter::new();
    assert_eq!(obj.ping(1), 1);
    assert_eq!(obj.ping(100), 2);
    assert_eq!(obj.ping(3001), 3);
    assert_eq!(obj.ping(3002), 3);
}
