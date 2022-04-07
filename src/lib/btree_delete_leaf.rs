use super::BTree;
use crate::{Node, NodeRef};
use std::cell::RefMut;

impl BTree {
    /// The logic to delete a leaf node
    pub(super) fn delete_leaf(node: &mut NodeRef, key_index: usize) {
        let mut current_node = node.borrow_mut();
        current_node.keys.remove(key_index);

        if current_node.has_more_than_min_keys() || current_node.has_min_key_count() {
            return;
        }

        let parent_weak: NodeRef = current_node.parent.upgrade().unwrap();
        let mut parent = parent_weak.borrow_mut();

        // Try and borrow a key from left sibling
        match current_node.try_clone_left_sibling() {
            Some(left_ref) if left_ref.borrow().has_more_than_min_keys() => {
                let mut left = left_ref.borrow_mut();
                BTree::move_from_left(&mut left, &mut parent, &mut current_node);
                return;
            }
            _ => {}
        }

        // Try and borrow a key from right sibling
        match current_node.try_clone_right_sibling() {
            Some(right_ref) if right_ref.borrow().has_more_than_min_keys() => {
                let mut right = right_ref.borrow_mut();
                BTree::move_from_right(&mut right, &mut parent, &mut current_node);
                return;
            }
            _ => {}
        }

        // Try and merge with rhe left sibling
        if let Some(left_ref) = current_node.try_clone_left_sibling() {
            let mut left_sibling = left_ref.borrow_mut();
            BTree::merge_with_left(&mut left_sibling, &mut parent, &mut current_node);
            return;
        }

        // Try and merge with the right sibling
        if let Some(right_ref) = current_node.try_clone_right_sibling() {
            let mut right_sibling = right_ref.borrow_mut();
            BTree::merge_with_right(&mut right_sibling, &mut parent, &mut current_node);
        }
    }

    /// Get the largest key from  the left sibling and pass it to the parent
    /// and take the parent's key to the left of the `moved_to` child and
    /// move it into it
    fn move_from_left(
        left: &mut RefMut<Node>,
        parent: &mut RefMut<Node>,
        moved_to: &mut RefMut<Node>,
    ) {
        let left_key = left.keys.pop().unwrap();
        let parent_key_to_rotate = parent.keys.remove(moved_to.index_in_parent.unwrap());

        parent.add_key(left_key);
        moved_to.add_key(parent_key_to_rotate);
    }

    /// Get the smallest key from  the right sibling and pass it to the parent
    /// and take the parent's key to the right of the `moved_to` child and
    /// move it into it
    fn move_from_right(
        right: &mut RefMut<Node>,
        parent: &mut RefMut<Node>,
        moved_to: &mut RefMut<Node>,
    ) {
        let right_key = right.keys.remove(0);
        let parent_key_to_rotate = parent.keys.remove(moved_to.index_in_parent.unwrap());

        parent.add_key(right_key);
        moved_to.add_key(parent_key_to_rotate);
    }

    fn merge_with_left(
        left_sibling: &mut RefMut<Node>,
        parent: &mut RefMut<Node>,
        moved_to: &mut RefMut<Node>,
    ) {
        let parent_key = parent.keys.remove(moved_to.index_in_parent.unwrap());

        left_sibling.add_key(parent_key);
        left_sibling.merge_node(moved_to);
    }

    fn merge_with_right(
        right_sibling: &mut RefMut<Node>,
        parent: &mut RefMut<Node>,
        moved_to: &mut RefMut<Node>,
    ) {
        let parent_key = parent.keys.remove(moved_to.index_in_parent.unwrap() + 1);

        right_sibling.add_key(parent_key);
        right_sibling.merge_node(moved_to);
    }
}

#[cfg(test)]
mod tests {
    use crate::BTree;

    #[test]
    fn test_simple_leaf_delete() {
        let mut tree = BTree::new(3);
        let _ = tree.add(0);
        let _ = tree.add(5);
        let _ = tree.add(10);
        let _ = tree.add(15);
        let _ = tree.add(1);



    }

    #[test]
    fn test_leaf_delete_with_left_move() {

    }

    #[test]
    fn test_leaf_delete_with_right_move() {

    }

    #[test]
    fn test_leaf_delete_with_left_merge() {

    }

    #[test]
    fn test_leaf_delete_with_right_merge() {

    }
}
