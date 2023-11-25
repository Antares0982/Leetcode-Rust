// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;


#[derive(Default)]
pub struct TreePalindromicInfo {
    pub st: [i32; 9],
    pub odd_counter: i32,
    pub ret: i32,
}

impl TreePalindromicInfo {
    pub fn process(&mut self, node: &Rc<RefCell<TreeNode>>) {
        let node = node.borrow();
        let node_val = node.val;
        // push
        {
            let k = (node_val - 1) as usize;
            self.odd_counter += 1 - 2 * (self.st[k] & 1);
            self.st[k] += 1;
        }
        // process
        if node.left.is_some() || node.right.is_some() {
            if let Some(left) = &node.left {
                self.process(left);
            }
            if let Some(right) = &node.right {
                self.process(right);
            }
        } else {
            // the final node
            if self.odd_counter <= 1 {
                self.ret += 1;
            }
        }
        // pop
        {
            let k = (node_val - 1) as usize;
            self.odd_counter += 1 - 2 * (self.st[k] & 1);
            self.st[k] -= 1;
        }
    }
}

impl Solution {
    pub fn pseudo_palindromic_paths(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut info = TreePalindromicInfo::default();
        if let Some(x) = root {
            info.process(&x);
        }
        info.ret
    }
}
