use crate::{Node, NodeRef};
use std::cell::RefMut;

/// The logic to delete a leaf node
pub(super) fn delete_leaf(parent: NodeRef, child_index: usize) {
    let mut parent = parent.borrow_mut();

    // Try and get a key from left
    if child_index != 0 {
        let left_idx = child_index - 1;
        if move_from_left(&mut parent, left_idx, child_index) {
            return;
        }
    }

    let max_size = parent.children.len();
    if child_index < max_size {
        let right_idx = child_index + 1;
        if move_from_right(&mut parent, right_idx, child_index) {
            return;
        }
    }

    // Try and merge with rhe left sibling
    // if let Some(left_ref) = child_node.try_clone_left_sibling() {
    //     //merge_with_left( &mut left_ref.borrow_mut(), &mut child_node);
    //     return;
    // }

    // Try and merge with the right sibling
    // if let Some(right_ref) = child_node.try_clone_right_sibling() {
    //     let mut right_sibling = right_ref.borrow_mut();
    //     merge_with_right(&mut right_sibling, &mut child_node);    }
}

// TODO: Combine with merge_from_right
/// Get the largest key from  the left sibling and pass it to the parent
/// and take the parent's key to the left of the `moved_to` child and
/// move it into it
fn move_from_left(parent: &mut RefMut<Node>, left_idx: usize, moved_to_idx: usize) -> bool {
    let left = parent.try_clone_child(left_idx as isize).unwrap();
    let mut left = left.borrow_mut();

    if !left.has_more_than_min_keys() { return false; }

    let moved_to = parent.try_clone_child(moved_to_idx as isize).unwrap();
    let mut moved_to = moved_to.borrow_mut();

    let left_key = left.keys.pop().unwrap();
    let parent_key_to_rotate = parent.keys.remove(left_idx);

    parent.add_key(left_key);
    moved_to.add_key(parent_key_to_rotate);
    return true;
}

// TODO: Simplify and combine with merge from left
/// Get the smallest key from  the right sibling and pass it to the parent
/// and take the parent's key to the right of the `moved_to` child and
/// move it into it
fn move_from_right(parent: &mut RefMut<Node>, right_idx: usize, moved_to_idx: usize) -> bool {
    let right = parent.try_clone_child(right_idx as isize).unwrap();
    let mut right = right.borrow_mut();

    if !right.has_more_than_min_keys() { return false; }

    let moved_to = parent.try_clone_child(moved_to_idx as isize).unwrap();
    let mut moved_to = moved_to.borrow_mut();

    let right_key = right.keys.remove(0);
    let parent_key_to_rotate = parent.keys.remove(moved_to_idx);

    parent.add_key(right_key);
    moved_to.add_key(parent_key_to_rotate);

    return true;
}

fn merge_with_left(parent: &mut RefMut<Node>, left: &mut RefMut<Node>, moved_to: &mut RefMut<Node>) {
    let parent_key = parent.keys.remove(moved_to.index_in_parent.unwrap() - 1);

    left.add_key(parent_key);
    drop(parent);
    //left_sibling.merge_node(moved_to);
}

fn merge_with_right(right_sibling: &mut RefMut<Node>, moved_to: &mut RefMut<Node>) {
    let parent_weak: NodeRef = moved_to.parent.upgrade().unwrap();
    let mut parent = parent_weak.borrow_mut();
    let parent_key = parent.keys.remove(moved_to.index_in_parent.unwrap());

    right_sibling.add_key(parent_key);
    drop(parent);
    //right_sibling.merge_node(moved_to);
}

#[cfg(test)]
mod tests {}
