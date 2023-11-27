use std::collections::VecDeque;

#[derive(Default)]
struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self { FrontMiddleBackQueue::default() }

    fn push_front(&mut self, val: i32) {
        if self.left.len() == self.right.len() {
            match self.left.len() {
                0 => {
                    self.right.push_front(val);
                    return;
                }
                _ => { self.left_right(); }
            }
        }
        self.left.push_front(val);
    }

    fn push_middle(&mut self, val: i32) {
        if self.left.len() == self.right.len() {
            self.right.push_front(val);
        } else {
            self.left.push_back(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        if self.right.len() > self.left.len() {
            self.right_left();
        }
        self.right.push_back(val);
    }

    fn pop_front(&mut self) -> i32 {
        if let Some(ret) = self.left.pop_front() {
            if self.left.len() < self.right.len() - 1 {
                self.right_left();
            }
            return ret;
        }
        match self.right.pop_front() {
            Some(x) => x,
            _ => -1
        }
    }

    fn pop_middle(&mut self) -> i32 {
        if self.left.len() == self.right.len() {
            return match self.left.pop_back() {
                Some(x) => x,
                _ => -1
            };
        }
        match self.right.pop_front() {
            Some(x) => x,
            _ => panic!("Unreachable")
        }
    }

    fn pop_back(&mut self) -> i32 {
        match self.right.pop_back() {
            Some(x) => {
                if self.right.len() < self.left.len() {
                    self.left_right();
                }
                x
            }
            _ => -1
        }
    }

    fn left_right(&mut self) {
        if let Some(x) = self.left.pop_back() {
            self.right.push_front(x);
        }
    }

    fn right_left(&mut self) {
        if let Some(x) = self.right.pop_front() {
            self.left.push_back(x);
        }
    }
}
