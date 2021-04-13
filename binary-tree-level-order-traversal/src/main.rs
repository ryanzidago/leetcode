use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

// Definition of a binary tree node.
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
            right: None,
        }
    }
}

fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let mut result: Vec<Vec<i32>> = Vec::new();

    if let Some(node) = root {
        queue.push_back(node);
    }

    while !queue.is_empty() {
        let mut level: Vec<i32> = Vec::new();

        for _ in 0..queue.len() {
            if let Some(node) = queue.pop_front() {
                level.push(node.borrow().val);

                if let Some(left) = node.borrow().left.clone() {
                    queue.push_back(Rc::clone(&left));
                }

                if let Some(right) = node.borrow().right.clone() {
                    queue.push_back(Rc::clone(&right));
                }
            }
        }
        result.push(level.clone())
    }

    result
}

// TODO: add tests cases
