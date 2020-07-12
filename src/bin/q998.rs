// Definition for a binary tree node.
struct Solution {}
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::{Ref, RefCell};
impl Solution {
    pub fn insert_into_max_tree(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Self::insert(root, val))
    }

    fn insert(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Rc<RefCell<TreeNode>> {
        let res = Rc::new(RefCell::new(TreeNode::new(val)));
        match root {
            None => (),
            Some(h) => {
                let old_val = h.borrow().val;
                if old_val < val {
                    res.borrow_mut().left = Some(h.clone())
                } else if old_val == val  {
                } else {
                    let (left, right) = Ref::map_split(h.borrow(), |n| (&n.left, &n.right));
                    {
                        let mut res_m = res.borrow_mut();
                        res_m.left = left.clone();
                        res_m.val = old_val;
                        res_m.right = Some(Self::insert(right.clone(), val));
                    }
                }
            }
        }
        res
    }
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    if let Some(node) = Solution::insert_into_max_tree(Some(root), 2) {
        println!("{:?}", node);
    }
}
