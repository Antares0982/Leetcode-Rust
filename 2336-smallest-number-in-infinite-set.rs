use std::collections::{BTreeSet, HashSet};
#[derive(Default)]
struct SmallestInfiniteSet {
    highest : i32,
    others: BTreeSet<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl SmallestInfiniteSet {
    fn new() -> Self {
        let mut ret = SmallestInfiniteSet::default();
        ret.highest = 1;
        ret
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.others.len() == 0 {
            self.highest += 1;
            return self.highest - 1;
        }
        if let Some(&first) = self.others.iter().next() {
            self.others.remove(&first);
            return first;
        }
        panic!("Unreachable")
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.highest {
            return;
        }
        self.others.insert(num);
        if num == self.highest - 1 {
            self._reform();
        }
    }
    fn _reform(&mut self) {
        let mut last = self.highest;
        let mut count = 0;
        let mut another_set: HashSet<i32> = HashSet::new();
        for value in self.others.iter().rev() {
            if *value == last - 1 {
                last = value.clone();
                another_set.insert(value.clone());
                count += 1;
                continue;
            }
            break;
        }

        for val in another_set.iter() {
            self.others.remove(val);
        }
        self.highest -= count;
    }
}



/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
