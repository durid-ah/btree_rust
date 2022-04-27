use std::cell::RefMut;
use crate::Node;

pub(super) fn delete_inner(deleted_key_node: &mut RefMut<Node>, deleted_key_index: usize) {

   let left_child_ref = deleted_key_node
         .try_clone_child(deleted_key_index as isize - 1);

   match left_child_ref {
      Some(left_child) if left_child.borrow_mut().has_more_than_min_keys() => {
         let mut left_child = left_child.borrow_mut();
         let child_key = left_child.keys.pop().unwrap();
         deleted_key_node.add_key(child_key);
      },
      _ => ()
   }

   let right_child_ref = deleted_key_node
      .try_clone_child(deleted_key_index as isize);

   match right_child_ref {
      Some(right_child) if right_child.borrow_mut().has_more_than_min_keys() => {
         let mut left_child = right_child.borrow_mut();
         let child_key = left_child.keys.pop().unwrap();
         deleted_key_node.add_key(child_key);
      },
      _ => ()
   }
}