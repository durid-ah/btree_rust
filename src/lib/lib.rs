use std::cell::RefCell;
use std::rc::Rc;
use node::{Node, NodeRef};

mod node;

const ALREADY_EXISTS_ERROR: &str = "Value already exists";

pub struct BTree {
   order: usize,
   root: NodeRef,
   child_min: usize,
   key_min: usize
}

impl BTree {
   pub fn new(order: usize) -> Self {
      let child_min = (order as f64 / 2.0).ceil() as usize;
      let key_min = child_min - 1;

      return Self {
         order,
         root: Rc::new(RefCell::new(Node::new(order))),
         child_min,
         key_min
      }
   }

   /// Add a value into the tree or return an error if the value already exists
   /// Works by searching each node for a possible location in every node
   /// until there is no child to insert it in
   pub fn add(&mut self,value: usize) -> Result<(), String> {
      let node_res = self.find_insert_node(value);

      if node_res.is_err() {
         return Err(node_res.unwrap_err());
      }

      let mut node = node_res.unwrap();

      node.borrow_mut().add_key(value);

      self.split_if_full(&mut node);

      return Ok(());
   }

   /// Get the node were you would insert the desired value
   fn find_insert_node(&mut self, value: usize) -> Result<NodeRef, String> {
      let mut node: NodeRef = Rc::clone(&self.root);

      loop {
         let res = (*node).borrow_mut()
            .find_future_key_index(value);
         if res.is_err() { return Err(String::from(ALREADY_EXISTS_ERROR)); }

         let child_idx = res.unwrap();
         let node_option = (*node).borrow_mut()
            .get_child(child_idx);
         match node_option {
            None => break,
            Some(child) => node = child
         }
      }

      return Ok(node);
   }

   fn split_if_full(&self, node: &mut NodeRef) {
      let node_ref = node.borrow_mut();
      let key_max = self.order - 1;

      if node_ref.key_count() <= key_max { return; }

      // TODO: Execute split on node
      // TODO: Insert key and children into parent
      // TODO: The split nodes have to be separate for insertion into parent
      // TODO: Loop again
      // TODO: See Node for split method
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

      BTree {order: 3, root, child_min: 0, key_min: 0}
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
}