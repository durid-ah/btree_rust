use crate::{Node, NodeRef};
use std::cell::RefMut;

/// The logic to delete a leaf node
pub(super) fn delete_leaf(parent: NodeRef, child_index: usize) {
    let mut parent = parent.borrow_mut();

    // Try and get a key from left
    if child_index != 0 {
        let left_idx = child_index - 1;
        if shift_key_from_sibling(&mut parent, left_idx, child_index) {
            return;
        }
    }

    let max_size = parent.children.len();
    if child_index < max_size {
        let right_idx = child_index + 1;
        if shift_key_from_sibling(&mut parent, right_idx, child_index) {
            return;
        }
    }

    // Try and merge with rhe left sibling
    if child_index != 0 {
        let _ = parent.merge_children(child_index - 1, child_index);
        return;
    }

    // Try and merge with the right sibling
    let _ = parent.merge_children(child_index + 1, child_index);
}

/// Shift a key from child in moved_from_idx into parent and the key in parent into
fn shift_key_from_sibling(
    parent: &mut RefMut<Node>, moved_from_idx: usize, moved_to_idx: usize) -> bool {
    let move_from_child = parent.try_clone_child(moved_from_idx as isize).unwrap();
    let mut move_from_child = move_from_child.borrow_mut();

    if !move_from_child.has_more_than_min_keys() { return false; }

    let moved_to = parent.try_clone_child(moved_to_idx as isize).unwrap();
    let mut moved_to = moved_to.borrow_mut();

    let (parent_key_idx, child_key_idx_to_move) = if moved_from_idx > moved_to_idx {
        // the moved_from is to the right
        (moved_to_idx, 0)
    } else {
        (moved_from_idx, move_from_child.keys.len() - 1)
    };

    let move_from_key = move_from_child.keys.remove(child_key_idx_to_move);
    let parent_key_to_rotate = parent.keys.remove(parent_key_idx);

    parent.add_key(move_from_key);
    moved_to.add_key(parent_key_to_rotate);
    return true;
}

#[cfg(test)]
mod tests {}
