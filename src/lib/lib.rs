use crate::node::search_status::SearchStatus;
use crate::BTreeError::ValueAlreadyExists;
use btree_delete_leaf as leaf_delete;
use node::{node_utils::new_node_ref, Node, NodeRef};
use std::rc::Rc;

mod node;
mod btree_delete_leaf;

#[derive(Debug)]
pub enum BTreeError {
    ValueAlreadyExists,
}

pub struct BTree {
    root: NodeRef,
    order: usize,
}

impl BTree {
    pub fn new(order: usize) -> Self {
        Self {
            root: new_node_ref(order),
            order,
        }
    }

    /// Add a value into the tree or return an error if the value already exists
    /// Works by searching each node for a possible location in every node
    /// until there is no child to insert it in
    pub fn add(&mut self, value: usize) -> Result<(), BTreeError> {
        let node_res = self.find_insert_node(value);

        if let Err(err) = node_res {
            return Err(err);
        }

        let node = node_res.unwrap();
        node.borrow_mut().add_key(value);

        self.split_if_full(node);

        Ok(())
    }

    pub fn delete(&mut self, value: usize) -> Result<(), BTreeError> {
        let (status, node_to_delete) = self.find(value);

        if !status.is_found() {
            return Ok(());
        }

        let mut node_ref = node_to_delete.borrow_mut();
        if node_ref.is_leaf() {
            // Leaf Node Cases
            leaf_delete::delete_leaf(&mut node_ref, status.unwrap());
            return Ok(());
        }

        // TODO:
        //    * if it does have children
        //       - bring up the left or right child key
        //       - if both left and right have minimum merge them together
        //       and if the node with deleted node still has minimum keys
        //       bring up left or right
        //    * if deletion affects height use parent and sibling to merge nodes together
        Err(BTreeError::ValueAlreadyExists)
    }

    fn find(&mut self, value: usize) -> (SearchStatus, NodeRef) {
        let mut node: NodeRef = Rc::clone(&self.root);
        let mut res = node.borrow_mut().find_key_index(value);
        loop {
            if res.is_found() {
                return (res, node);
            }

            let child_idx = res.unwrap() as isize;
            let node_option = node.borrow_mut().try_clone_child(child_idx);

            match node_option {
                None => break,
                Some(child) => {
                    node = child;
                    res = node.borrow_mut().find_key_index(value);
                }
            }
        }

        (res, node)
    }

    /// Get the node were you would insert the desired value
    fn find_insert_node(&mut self, value: usize) -> Result<NodeRef, BTreeError> {
        let (status, insert_node) = self.find(value);

        if status.is_found() {
            return Err(ValueAlreadyExists);
        }

        Ok(insert_node)
    }

    fn split_if_full(&mut self, node: NodeRef) {
        let mut node_ref = Rc::clone(&node);

        loop {
            if !node_ref.borrow_mut().is_key_overflowing() {
                break;
            }

            let (mid_key, right_node) = node_ref.borrow_mut().split_node();
            let parent_option: Option<NodeRef> = node_ref.borrow_mut().parent.upgrade();
            let mut insert_left = false;

            let parent: NodeRef = match parent_option {
                Some(node_ref) => Rc::clone(&node_ref),
                None => {
                    // if we are splitting the root node instantiate a new parent
                    let new_parent: NodeRef = new_node_ref(self.order);
                    self.root = Rc::clone(&new_parent); // set the new parent as the root
                                                        // if the parent is new the left node needs to be inserted
                    insert_left = true;
                    new_parent
                }
            };

            let mut parent_node = parent.borrow_mut();

            right_node.borrow_mut().parent = Rc::downgrade(&parent);
            node_ref.borrow_mut().parent = Rc::downgrade(&parent);

            parent_node.add_key(mid_key);
            if insert_left {
                parent_node.add_child(Rc::clone(&node_ref)); // left node
            }
            parent_node.add_child(right_node); // right node
            node_ref = Rc::clone(&parent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BTree;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn build_tree() -> BTree {
        let left_child = Rc::new(RefCell::new(Node::new(3)));

        left_child.borrow_mut().add_key(1);
        left_child.borrow_mut().add_key(3);

        let right_child = Rc::new(RefCell::new(Node::new(3)));

        right_child.borrow_mut().add_key(7);
        right_child.borrow_mut().add_key(9);

        let root = Rc::new(RefCell::new(Node::new(3)));

        root.borrow_mut().add_key(5);

        root.borrow_mut().children.push(left_child);
        root.borrow_mut().children.push(right_child);

        BTree { root, order: 3 }
    }

    #[test]
    fn test_find_node() {
        let mut tree = build_tree();
        let left_node_test = tree.find_insert_node(2).unwrap();
        let right_node_test = tree.find_insert_node(8).unwrap();

        assert_eq!(left_node_test.borrow_mut().keys, vec![1, 3]);
        assert_eq!(right_node_test.borrow_mut().keys, vec![7, 9]);

        let left_node_test = tree.find_insert_node(4).unwrap();
        let right_node_test = tree.find_insert_node(6).unwrap();

        assert_eq!(left_node_test.borrow_mut().keys, vec![1, 3]);
        assert_eq!(right_node_test.borrow_mut().keys, vec![7, 9]);
    }

    mod add_key_tests {
        use super::*;

        #[test]
        fn test_add_node() {
            let mut tree = BTree::new(3);
            let _ = tree.add(1);
            let _ = tree.add(2);
            let _ = tree.add(3);
            let _ = tree.add(4);

            let root_ref = tree.root;
            let root = root_ref.borrow_mut();

            assert_eq!(root.keys.len(), 1);
            assert_eq!(root.keys[0], 2);
            assert_eq!(root.children.len(), 2);

            let first_child = root.children[0].borrow();
            assert_eq!(first_child.keys[0], 1);
            assert_eq!(first_child.keys.len(), 1);

            let second_child = root.children[1].borrow();
            assert_eq!(second_child.keys[0], 3);
            assert_eq!(second_child.keys[1], 4);
            assert_eq!(second_child.keys.len(), 2);
        }

        #[test]
        fn test_out_of_order_add() {
            let mut tree = BTree::new(3);
            let _ = tree.add(4);
            let _ = tree.add(2);
            let _ = tree.add(1);
            let _ = tree.add(3);

            let root_ref = tree.root;
            let root = root_ref.borrow_mut();

            assert_eq!(root.keys.len(), 1);
            assert_eq!(root.keys[0], 2);
            assert_eq!(root.children.len(), 2);

            let first_child = root.children[0].borrow();
            assert_eq!(first_child.keys[0], 1);
            assert_eq!(first_child.keys.len(), 1);

            let second_child = root.children[1].borrow();
            assert_eq!(second_child.keys[0], 3);
            assert_eq!(second_child.keys[1], 4);
            assert_eq!(second_child.keys.len(), 2);
        }

        #[test]
        fn test_out_two_splits() {
            let mut tree = BTree::new(3);
            let _ = tree.add(4);
            let _ = tree.add(2);
            let _ = tree.add(1);
            let _ = tree.add(3);
            let _ = tree.add(5);

            let root_ref = tree.root;
            let root = root_ref.borrow_mut();

            assert_eq!(root.keys.len(), 2);
            assert_eq!(root.keys[0], 2);
            assert_eq!(root.children.len(), 3);

            let first_child = root.children[0].borrow();
            assert_eq!(first_child.keys[0], 1);
            assert_eq!(first_child.keys.len(), 1);

            let second_child = root.children[1].borrow();
            assert_eq!(second_child.keys[0], 3);
            assert_eq!(second_child.keys.len(), 1);

            let third_child = root.children[2].borrow();
            assert_eq!(third_child.keys[0], 5);
            assert_eq!(third_child.keys.len(), 1);
        }

        #[test]
        fn test_out_three_levels() {
            let mut tree = BTree::new(3);
            let _ = tree.add(1);
            let _ = tree.add(2);
            let _ = tree.add(3);
            let _ = tree.add(4);
            let _ = tree.add(5);
            let _ = tree.add(6);
            let _ = tree.add(7);

            let root_ref = tree.root;
            let root = root_ref.borrow_mut();

            assert_eq!(root.keys.len(), 1);
            assert_eq!(root.keys[0], 4);
            assert_eq!(root.children.len(), 2);

            let first_child = root.children[0].borrow();
            assert_eq!(first_child.keys[0], 2);
            assert_eq!(first_child.keys.len(), 1);
            assert_eq!(first_child.children.len(), 2);

            let level_3_first_child = first_child.children[0].borrow();
            assert_eq!(level_3_first_child.keys[0], 1);
            assert_eq!(level_3_first_child.keys.len(), 1);

            let level_3_second_child = first_child.children[1].borrow();
            assert_eq!(level_3_second_child.keys[0], 3);
            assert_eq!(level_3_second_child.keys.len(), 1);

            let second_child = root.children[1].borrow();
            assert_eq!(second_child.keys[0], 6);
            assert_eq!(second_child.keys.len(), 1);

            let level_3_first_child = second_child.children[0].borrow();
            assert_eq!(level_3_first_child.keys[0], 5);
            assert_eq!(level_3_first_child.keys.len(), 1);

            let level_3_second_child = second_child.children[1].borrow();
            assert_eq!(level_3_second_child.keys[0], 7);
            assert_eq!(level_3_second_child.keys.len(), 1);
        }
    }

    mod delete_key_tests {
        use super::*;

        #[test]
        fn test_simple_leaf_delete() {
            let mut tree = BTree::new(3);
            let _ = tree.add(0);
            let _ = tree.add(5);
            let _ = tree.add(10);
            let _ = tree.add(15);
            let _ = tree.add(1);

            let res = tree.delete(15);
            assert!(res.is_ok());
            let (res, _) = tree.find(15);
            match res {
                SearchStatus::NotFound(_) => assert!(true),
                SearchStatus::Found(_) => assert!(false, "Key 15 should be deleted")
            }

            let root = tree.root.borrow_mut();
            let key_vec = &root.keys;
            assert_eq!(*key_vec, vec![5]);

            let left_child = root.children[0].borrow_mut();
            let left_child_keys = &left_child.keys;
            assert_eq!(*left_child_keys, vec![0, 1]);

            let right_child = root.children[1].borrow_mut();
            let right_child_keys = &right_child.keys;
            assert_eq!(*right_child_keys, vec![10]);
        }

        #[test]
        fn test_leaf_delete_with_left_move() {
            let mut tree = BTree::new(3);
            let _ = tree.add(0);
            let _ = tree.add(5);
            let _ = tree.add(10);
            let _ = tree.add(15);
            let _ = tree.add(1);

            let _ = tree.delete(15);
            let res = tree.delete(10);
            assert!(res.is_ok());
            let (res, _) = tree.find(10);
            match res {
                SearchStatus::NotFound(_) => assert!(true),
                SearchStatus::Found(_) => assert!(false, "Key 15 should be deleted")
            }

            let root = tree.root.borrow_mut();
            let key_vec = &root.keys;
            assert_eq!(*key_vec, vec![1]);

            let left_child = root.children[0].borrow_mut();
            let left_child_keys = &left_child.keys;
            assert_eq!(*left_child_keys, vec![0]);

            let right_child = root.children[1].borrow_mut();
            let right_child_keys = &right_child.keys;
            assert_eq!(*right_child_keys, vec![5]);
        }

        #[test]
        fn test_leaf_delete_with_right_move() {

        }

        #[test]
        fn test_leaf_delete_with_left_merge() {}

        #[test]
        fn test_leaf_delete_with_right_merge() {}
    }
}
