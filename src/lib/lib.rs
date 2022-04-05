use std::cell::{RefCell, RefMut};
use std::rc::{Rc};
use node::{Node, NodeRef, node_utils::new_node_ref};
use crate::BTreeError::ValueAlreadyExists;
use crate::node::search_status::SearchStatus;

mod node;

#[derive(Debug)]
pub enum BTreeError {
   ValueAlreadyExists
}

pub struct BTree {
   root: NodeRef,
   order: usize
}

impl BTree {
   pub fn new(order: usize) -> Self {
      Self { root: new_node_ref(order), order }
   }

   /// Add a value into the tree or return an error if the value already exists
   /// Works by searching each node for a possible location in every node
   /// until there is no child to insert it in
   pub fn add(&mut self,value: usize) -> Result<(), BTreeError> {
      let node_res = self.find_insert_node(value);

      if node_res.is_err() {
         return Err(node_res.unwrap_err());
      }

      let node = node_res.unwrap();
      node.borrow_mut().add_key(value);

      self.split_if_full(node);

      return Ok(());
   }

   /// The logic to delete a leaf node
   fn delete_leaf(node:&mut NodeRef, key_index: usize) {
      let mut current_node = node.borrow_mut();
      let current_node_idx = current_node.index_in_parent.unwrap();
      current_node.keys.remove(key_index);

      if current_node.has_more_than_min_keys() || current_node.has_min_key_count() {
         return;
      }

      let parent_weak: NodeRef = current_node.parent.upgrade().unwrap();
      let mut parent = parent_weak.borrow_mut();

      let mut moved = Err(());

      if current_node_idx != 0 {
         moved = BTree::move_from_left(&mut parent, &mut current_node);
      }

      if moved.is_ok() { return; }

      if current_node_idx < parent.children.len() - 1 {
         moved = BTree::move_from_right(&mut parent, &mut current_node);
      }

      if moved.is_ok() { return; }

      if current_node_idx != 0 {
         BTree::merge_with_left(&mut parent, &mut current_node);
         return;
      }

      if current_node_idx < parent.children.len() - 1 {
         BTree::merge_with_right(&mut parent, &mut current_node);
         return;
      }

   }

   fn move_from_left(parent: &mut RefMut<Node>, moved_to: &mut RefMut<Node>) -> Result<(),()> {
      let left_ref = moved_to.try_clone_left_sibling().unwrap();
      let mut left_sibling = left_ref.borrow_mut();

      if left_sibling.has_more_than_min_keys() {
         let left_key = left_sibling.keys.pop().unwrap();
         let parent_key_to_rotate = parent.keys
            .remove(moved_to.index_in_parent.unwrap());

         parent.add_key(left_key);
         moved_to.add_key(parent_key_to_rotate);
         return Ok(());
      }

      Err(())
   }

   fn move_from_right(parent: &mut RefMut<Node>, moved_to: &mut RefMut<Node>) -> Result<(),()> {
      let right_ref = moved_to.try_clone_right_sibling()
         .unwrap();
      let mut right_sibling = right_ref.borrow_mut();

      if right_sibling.has_more_than_min_keys() {
         let right_key = right_sibling.keys.remove(0);
         let parent_key_to_rotate = parent.keys
            .remove(moved_to.index_in_parent.unwrap());

         parent.add_key(right_key);
         moved_to.add_key(parent_key_to_rotate);
         return Ok(())
      }

      Err(())
   }

   fn merge_with_left(parent: &mut RefMut<Node>, moved_to: &mut RefMut<Node>) {
      let left_ref = moved_to.try_clone_left_sibling()
         .unwrap();
      let mut left_sibling = left_ref.borrow_mut();
      let total_capacity = left_sibling.keys.len() + moved_to.keys.len();
      let mut merged_vec: Vec<usize> = Vec::with_capacity(total_capacity);
      let parent_key = parent.keys
         .remove(moved_to.index_in_parent.unwrap());

      merged_vec.append(&mut left_sibling.keys);
      merged_vec.push(parent_key);
      merged_vec.append(&mut moved_to.keys);
      merged_vec.sort();

      left_sibling.keys = merged_vec;
      parent.remove_child(moved_to.index_in_parent.unwrap());
   }

   fn merge_with_right(parent: &mut RefMut<Node>, moved_to: &mut RefMut<Node>) {
      let right_ref = moved_to.try_clone_right_sibling()
         .unwrap();
      let mut right_sibling = right_ref.borrow_mut();
      let total_capacity = right_sibling.keys.len() + moved_to.keys.len();
      let mut merged_vec: Vec<usize> = Vec::with_capacity(total_capacity);
      let parent_key = parent.keys
         .remove(moved_to.index_in_parent.unwrap() + 1);

      merged_vec.append(&mut right_sibling.keys);
      merged_vec.push(parent_key);
      merged_vec.append(&mut moved_to.keys);
      merged_vec.sort();

      right_sibling.keys = merged_vec;
      parent.remove_child(moved_to.index_in_parent.unwrap());
   }

   pub fn delete(&mut self, value: usize) -> Result<(), BTreeError> {
      let (status, mut node_to_delete) = self.find(value);

      if !status.is_found() { return Ok(()); }

      if !node_to_delete.borrow_mut().is_leaf() { // Leaf Node Cases
         BTree::delete_leaf( &mut node_to_delete, status.unwrap());
         return  Ok(())
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
      let res = node.borrow_mut().find_key_index(value);
      loop {
         if res.is_found() { return (res, node); }

         let child_idx = res.unwrap() as isize;
         let node_option = node.borrow_mut()
            .try_clone_child(child_idx);

         match node_option {
            None => break,
            Some(child) => node = child
         }
      }

      return  (res, node)
   }

   /// Get the node were you would insert the desired value
   fn find_insert_node(&mut self, value: usize) -> Result<NodeRef, BTreeError> {
      let (status, insert_node) = self.find(value);

      if status.is_found() { return  Err(ValueAlreadyExists) }

      return Ok(insert_node);
   }

   fn split_if_full(&mut self, node: NodeRef) {
      let mut node_ref = Rc::clone(&node);

      loop {
         if !node_ref.borrow_mut().is_key_overflowing() { break; }

         let (mid_key, right_node) = node_ref.borrow_mut().split_node();
         let parent_option: Option<NodeRef> = node_ref.borrow_mut().parent.upgrade();
         let mut insert_left = false;

         let parent: NodeRef = match parent_option {
            Some(node_ref) => Rc::clone(&node_ref),
            None => {
               // if we are splitting the root node instantiate a new parent
               let new_parent :NodeRef = new_node_ref(self.order);
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
   use std::rc::Rc;
   use crate::BTree;
   use super::*;

   fn build_tree() -> BTree {
      let left_child = Rc::new(
         RefCell::new(
            Node::new(3)));

      left_child.borrow_mut().add_key(1);
      left_child.borrow_mut().add_key(3);

      let right_child = Rc::new(
         RefCell::new(
            Node::new(3)));

      right_child.borrow_mut().add_key(7);
      right_child.borrow_mut().add_key(9);

      let root = Rc::new(
         RefCell::new(
            Node::new(3)));

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
}