use std::cell::RefCell;
use std::rc::{Rc};
use node::{Node, NodeRef, new_node_ref};

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
      return Self {
         root: new_node_ref(order),
         order
      }
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

   /// Get the node were you would insert the desired value
   fn find_insert_node(&mut self, value: usize) -> Result<NodeRef, BTreeError> {
      let mut node: NodeRef = Rc::clone(&self.root);

      loop {
         let res = (*node.borrow_mut())
            .find_future_key_index(value);
         if res.is_err() { return Err(BTreeError::ValueAlreadyExists); }

         let child_idx = res.unwrap();
         let node_option = (*node.borrow_mut())
            .get_child(child_idx);

         match node_option {
            None => break,
            Some(child) => node = child
         }
      }

      return Ok(node);
   }

   // TODO: Fix the duplicate left node
   // TODO: Use Rc::ptr_eq? in the add child?
   // TODO: Avoid insertion in the first place?
   fn split_if_full(&mut self, node: NodeRef) {
      let mut node_ref = Rc::clone(&node);

      loop {
         if !node_ref.borrow_mut().is_key_overflowing() { break; }

         let (mid_key, mut right_node) = node_ref.borrow_mut().split_node();
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

         right_node.parent = Rc::downgrade(&parent);
         node_ref.borrow_mut().parent = Rc::downgrade(&parent);

         parent_node.add_key(mid_key);
         if insert_left {
            parent_node.add_child(Rc::clone(&node)); // left node
         }
         parent_node.add_child(Rc::new(RefCell::new(right_node))); // right node
         node_ref = Rc::clone(&parent)
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

         // assert_eq!(second_child.keys[1], 4);

      }
   }


}