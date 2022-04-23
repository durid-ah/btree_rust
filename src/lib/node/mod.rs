use node_utils::{calculate_mid, new_node_ref};
use search_status::SearchStatus;
use std::cell::{RefCell};
use std::rc::{Rc, Weak};

pub(crate) mod node_child_operations;
pub(crate) mod node_utils;
pub(crate) mod search_status;

pub(crate) type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

/// # Node Rules:
/// * Max number of keys (order - 1)
/// * Min number of keys `ceil(order/2) - 1`
/// * Min number of children `ceil(order/2)`
#[derive(Debug)]
pub(crate) struct Node {
    pub parent: WeakNodeRef,
    pub index_in_parent: Option<usize>, // TODO: Very likely useless
    pub keys: Vec<usize>,
    pub children: Vec<NodeRef>,

    order: usize,
    min_keys: usize,
}

impl Node {
    pub fn new(order: usize) -> Self {
        Self {
            parent: Weak::new(),
            index_in_parent: Option::None,
            keys: Vec::with_capacity(order - 1),
            children: Vec::with_capacity(order),
            min_keys: (order as f32 / 2_f32).ceil() as usize - 1,
            order,
        }
    }

    pub fn add_key(&mut self, key: usize) {
        // add the new key at the end
        self.keys.push(key);
        let mut new_key_idx = self.keys.len() - 1;

        if new_key_idx == 0 {
            return;
        }

        // shift the key to the left until the values are in order
        let mut current_idx = new_key_idx - 1;
        while self.keys[new_key_idx] < self.keys[current_idx] {
            self.keys.swap(new_key_idx, current_idx);

            if current_idx > 0 {
                new_key_idx = current_idx;
                current_idx -= 1;
            }
        }
    }

    /// Find the index where the new key would reside or the place where it
    /// already exists
    ///
    /// # Returns
    /// Found(i: usize) => The value exists and `i` is the index location
    /// NotFound(i:usize) => The value does not exist and `i` is where the item should be
    pub fn find_key_index(&self, key: usize) -> SearchStatus {
        if self.keys.is_empty() {
            return SearchStatus::NotFound(0);
        }

        let mut start = 0_isize;
        let key_length = self.keys.len();
        let mut end = (key_length - 1) as isize;

        while end >= start {
            let mid: isize = calculate_mid(start, end);
            let mid_idx = mid as usize;

            if self.keys[mid_idx] == key {
                return SearchStatus::Found(mid_idx);
            } else if mid_idx == 0 && self.keys[mid_idx] > key {
                return SearchStatus::NotFound(mid_idx);
            } else if mid_idx == (key_length - 1) && self.keys[mid_idx] < key {
                return SearchStatus::NotFound(mid_idx + 1);
            } else if self.keys[mid_idx] > key && self.keys[mid_idx - 1] < key {
                return SearchStatus::NotFound(mid_idx);
            }

            if self.keys[mid_idx] > key {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        panic!("Unable to find value {}", key)
    }

    //TODO: Change to split child too?
    /// Split the node down the middle and return the mid key and right
    /// node that broke off
    ///
    /// # Returns
    /// (mid_key: usize, right_node: Node) => `mid_key` represents the key in the middle of
    /// node and `right_node` is the node broken off to the right
    pub fn split_node(&mut self) -> (usize, NodeRef) {
        let key_len = self.keys.len();
        let child_len = self.children.len();
        let mid_key_idx = key_len / 2;

        let right_node = new_node_ref(self.order);

        let mut right_keys = Vec::with_capacity(self.order - 1);
        let mut right_children = Vec::with_capacity(self.order);

        // pop half of the kids
        for _ in (mid_key_idx + 1)..key_len {
            let key = self.keys.pop().unwrap();
            right_keys.push(key);
        }
        right_keys.reverse(); // ensure they are in the proper order

        // pop half of the children
        for (right_child_idx, _) in ((mid_key_idx + 1)..child_len).rev().enumerate() {
            let node = self.children.pop().unwrap();
            node.borrow_mut().parent = Rc::downgrade(&right_node);
            node.borrow_mut().index_in_parent = Some(right_child_idx);
            right_children.push(node);
        }
        right_children.reverse(); // ensure they are in the proper order

        let mid_key = self.keys.pop().unwrap();

        right_node.borrow_mut().children = right_children;
        right_node.borrow_mut().keys = right_keys;
        right_node.borrow_mut().parent = self.parent.clone();

        (mid_key, right_node)
    }

    pub fn merge_children(&mut self, merge_into_index: usize, merge_from_index: usize) -> Result<(), String> {
        let diff = (merge_into_index as isize - merge_from_index as isize).abs();

        if diff != 1 {
            return Err(String::from("The children must be next to each other"));
        }

        let mut merge_into_child = self.children[merge_into_index].borrow_mut();
        let mut merge_from_child = self.children[merge_from_index].borrow_mut();

        merge_into_child.keys.append(&mut merge_from_child.keys);
        merge_into_child.children.append(&mut merge_from_child.children);

        drop(merge_into_child);
        drop(merge_from_child);

        self.children.remove(merge_from_index);

        Ok(())
    }

    /// Shows if the key container is over capacity and ready for a split
    pub fn is_key_overflowing(&self) -> bool {
        self.keys.len() > self.order - 1
    }

    /// Returns true if the node is the root and has 1 key
    /// has otherwise if it has ceil(order / 2) - 1 keys
    pub fn has_min_key_count(&self) -> bool {
        if self.is_root() {
            self.keys.len() == 1
        } else {
            self.keys.len() == self.min_keys
        }
    }

    pub fn has_more_than_min_keys(&self) -> bool {
        if self.is_root() {
            self.keys.len() > 1
        } else {
            self.keys.len() > self.min_keys
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent.upgrade().is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn get_key(&self, index: usize) -> usize {
        self.keys[index]
    }

    fn get_min_key(&self) -> usize {
        self.get_key(0)
    }

    fn get_max_key(&self) -> usize {
        self.get_key(self.keys.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;

    mod find_key_tests {
        use super::*;

        #[test]
        fn find_key_in_1_element() {
            let mut node = Node::new(5);
            node.keys.push(5);

            let res = node.find_key_index(5);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 0);

            let res = node.find_key_index(3);
            assert!(!res.is_found());
        }

        #[test]
        fn find_key_in_2_element() {
            let mut node = Node::new(5);
            node.keys.push(5);
            node.keys.push(7);

            let res = node.find_key_index(5);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 0);

            let res = node.find_key_index(7);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 1);

            let res = node.find_key_index(3);
            assert!(!res.is_found());

            let res = node.find_key_index(6);
            assert!(!res.is_found());

            let res = node.find_key_index(8);
            assert!(!res.is_found());
        }

        #[test]
        fn find_key_in_3_element() {
            let mut node = Node::new(8);
            node.keys = vec![5, 7, 9];

            let res = node.find_key_index(5);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 0);

            let res = node.find_key_index(7);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 1);

            let res = node.find_key_index(9);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 2);

            let res = node.find_key_index(3);
            assert!(!res.is_found());

            let res = node.find_key_index(6);
            assert!(!res.is_found());

            let res = node.find_key_index(8);
            assert!(!res.is_found());

            let res = node.find_key_index(10);
            assert!(!res.is_found());
        }

        #[test]
        fn find_key_in_4_element() {
            let mut node = Node::new(8);
            node.keys = vec![5, 7, 9, 11];

            let res = node.find_key_index(5);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 0);

            let res = node.find_key_index(7);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 1);

            let res = node.find_key_index(9);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 2);

            let res = node.find_key_index(11);
            assert!(res.is_found());
            assert_eq!(res.unwrap(), 3);

            let res = node.find_key_index(3);
            assert!(!res.is_found());

            let res = node.find_key_index(6);
            assert!(!res.is_found());

            let res = node.find_key_index(8);
            assert!(!res.is_found());

            let res = node.find_key_index(10);
            assert!(!res.is_found());

            let res = node.find_key_index(12);
            assert!(!res.is_found());
        }
    }

    mod find_location_tests {
        use super::*;
        use crate::node::SearchStatus;

        #[test]
        fn find_location_in_even_vector() {
            let mut node = Node::new(5);
            node.keys = vec![5, 10, 15, 20];

            match node.find_key_index(3) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 0, "Value must be 0 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(8) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 1, "Value must be 1 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(11) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 2, "Value must be 2 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(18) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 3, "Value must be 3 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(25) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 4, "Value must be 4 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }
        }

        #[test]
        fn find_location_in_odd_vector() {
            let mut node = Node::new(5);
            node.keys = vec![5, 10, 15, 20, 25];

            match node.find_key_index(3) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 0, "Value must be 0 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(8) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 1, "Value must be 1 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(11) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 2, "Value must be 2 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(18) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 3, "Value must be 3 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(23) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 4, "Value must be 4 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(26) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 5, "Value must be 5 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }
        }

        #[test]
        fn find_location_in_single_element() {
            let mut node = Node::new(5);
            node.keys = vec![5];

            match node.find_key_index(3) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 0, "Value must be 0 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }

            match node.find_key_index(8) {
                SearchStatus::NotFound(index) => {
                    assert_eq!(index, 1, "Value must be 1 instead got {}", index)
                }
                SearchStatus::Found(_) => assert!(false, "Value"),
            }
        }
    }

    mod split_nodes_tests {
        use super::*;

        #[test]
        fn split_nodes_with_odd_order() {
            let order = 3;
            let min_key = (order as f32 / 2.0).ceil() as usize - 1;

            let mut node = Node::new(order);
            node.keys.push(1);
            node.keys.push(2);
            node.keys.push(3);
            node.keys.push(4);

            let (mid_key, right) = node.split_node();

            assert!(node.keys.len() >= min_key);
            assert!(right.borrow().keys.len() >= min_key);

            assert_eq!(node.keys, vec![1, 2]);
            assert_eq!(right.borrow().keys, vec![4]);
            assert_eq!(mid_key, 3);
        }

        #[test]
        fn split_nodes_with_even_order() {
            let order = 4;
            let min_key = (order as f32 / 2.0).ceil() as usize - 1;

            let mut node = Node::new(order);
            node.keys.push(1);
            node.keys.push(2);
            node.keys.push(3);
            node.keys.push(4);
            node.keys.push(5);

            let (mid_key, right) = node.split_node();

            assert!(node.keys.len() >= min_key);
            assert!(right.borrow().keys.len() >= min_key);

            assert_eq!(node.keys, vec![1, 2]);
            assert_eq!(right.borrow().keys, vec![4, 5]);
            assert_eq!(mid_key, 3);
        }

        #[test]
        fn split_nodes_with_6_order() {
            let order = 6;
            let min_key = (order as f32 / 2.0).ceil() as usize - 1;

            let mut node = Node::new(order);
            node.keys.push(1);
            node.keys.push(2);
            node.keys.push(3);
            node.keys.push(4);
            node.keys.push(5);
            node.keys.push(6);

            let (mid_key, right) = node.split_node();

            assert!(node.keys.len() >= min_key);
            assert!(right.borrow().keys.len() >= min_key);
            assert_eq!(node.keys, vec![1, 2, 3]);
            assert_eq!(right.borrow().keys, vec![5, 6]);
            assert_eq!(mid_key, 4);
        }
    }
}
