
use crate::TreeNode;
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

struct Solution;
impl Solution {

    pub fn serialize(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_string();
        }

        let mut res = vec![];
        let mut dq = VecDeque::new();
        dq.push_back(root);

        while ! dq.is_empty() {
            let cnt = dq.len();
            {
                for _ in 0..cnt {
                    let mut root = dq.pop_front().unwrap();
                    res.push(root.as_ref().map(
                        |x| x.borrow().val.to_string()).unwrap_or(String::from("null")));

                    if let Some(node) = &mut root {
                        dq.push_back(node.borrow_mut().left.take());
                        dq.push_back(node.borrow_mut().right.take());
                    }
                }
            }
        }

        while res.last().unwrap_or(&String::from("xxx")) == &String::from("null") {
            res.pop();
        }

        res.join(",")
    }
    pub fn descrialize(data: String) -> Option<Rc<RefCell<TreeNode>>> {
        let mut item_iter = data.split(",");
        let root = if let Some(root_str) = item_iter.next() {
            Self::create_tree_node(root_str)
        } else {
            return None;
        };
        let mut dq = VecDeque::new();
        dq.push_back(root.clone());
        loop {
            let node = dq.pop_front().unwrap();
            if let Some(node_str) = item_iter.next() {
                let child = Self::create_tree_node(node_str);
                if let Some(n) = &child {
                    dq.push_back(Some(n.clone()));
                }
                node.as_ref().unwrap().borrow_mut().left = child;
            } else {
                break
            }
            if let Some(node_str) = item_iter.next() {
                let child = Self::create_tree_node(node_str);
                if let Some(n) = &child {
                    dq.push_back(Some(n.clone()));
                }
                node.as_ref().unwrap().borrow_mut().right = child;
            } else {
                break
            }
        }

        root
    }
    fn create_tree_node(root_str: &str) -> Option<Rc<RefCell<TreeNode>>> {
        root_str.parse::<i32>().ok().map(
            |x| Rc::new(RefCell::new(TreeNode::new(x))))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test1() {
        let array = [
            "1,2,3,4,5",
            "1,null,3,4,null,5",
            "1,null,3,null,4",
            "1,2,3,null,null,4",
            "1,2,3,null,null,null,4",
        ];
        for s in array.iter() {
            let root = Solution::descrialize(s.to_string());
            let new_s = Solution::serialize(root);
            assert_eq!(new_s, s.to_string());
        }
    }
}