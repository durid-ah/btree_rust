use std::cell::{RefCell};
use std::rc::{Rc, Weak};

pub type NodeRef = Rc<RefCell<Node>>;
type WeakNodeRef = Weak<RefCell<Node>>;

pub struct Node {
   pub parent : Option<WeakNodeRef>,
   pub keys: Vec<usize>,
   pub child_count: usize,

   children: Vec<Option<NodeRef>>,
   order: usize,
   min_child_count: usize,
}

impl Node {
   pub fn new(order: usize) -> Self {
      let min_child_count: usize = order / 2;

      return Self {
         parent: Option::None,
         keys: Vec::with_capacity(order - 1),
         children: vec![Option::None; order],
         child_count: 0,
         order,
         min_child_count
      }
   }

   pub fn add_key(&mut self, key: usize) {
      // add the new key at the end
      self.keys.push(key);
      let mut new_key_idx = self.keys.len() - 1;

      if new_key_idx == 0 { return; }

      // shift the key to the left until the values are in order
      let mut current_idx = new_key_idx - 1;
      while self.keys[new_key_idx] < self.keys[current_idx] {
         let temp = self.keys[current_idx];
         self.keys[current_idx] = self.keys[new_key_idx];
         self.keys[new_key_idx] = temp;

         if current_idx > 0 {
            new_key_idx = current_idx;
            current_idx -= 1;
         }
      }
   }

   /// Return index of the key if found or Option::None otherwise
   pub fn find_key(&self, key: usize) -> Option<usize> {
      let calculate_mid =
         |start: isize, end: isize| -> isize { ((end - start) / 2) + start };

      if self.keys.len() == 0 {
         return Option::None;
      }

      let mut start = 0 as isize;
      let mut end = (self.keys.len() - 1) as isize;

      while end >= start {
         let mid: isize = calculate_mid(start, end);
         let mid_idx = mid as usize;

         if self.keys[mid_idx] == key {
            return Option::Some(mid_idx);
         }

         if self.keys[mid_idx] > key {
            end = mid - 1;
         } else {
            start = mid + 1;
         }
      }

      return Option::None;
   }

   /// Find the index where the new key would reside or an error with the
   /// index where it already exists
   ///
   /// # Returns
   /// Ok(i: usize) => where `i` is the index location
   /// Err((i:usize, err:String)) => a tuple where `i` is the existing location and err is the message
   pub fn find_future_key_index(&self, key: usize) -> Result<usize, (usize, String)> {
      let calculate_mid =
         |start: isize, end: isize| -> isize { ((end - start) / 2) + start };

      if self.keys.len() == 0 {
         return Ok(0);
      }

      let mut start = 0 as isize;
      let key_length = self.keys.len();
      let mut end = (key_length - 1) as isize;

      while end >= start {
         let mid: isize = calculate_mid(start, end);
         let mid_idx = mid as usize;

         if self.keys[mid_idx] == key {
            return Err((mid_idx, format!("value already exists at {}", mid_idx)))
         }
         else if mid_idx == 0 && self.keys[mid_idx] > key {
            return Ok(mid_idx)
         }
         else if mid_idx == (key_length - 1) && self.keys[mid_idx] < key {
            return Ok(mid_idx + 1)
         }
         else if self.keys[mid_idx] > key && self.keys[mid_idx - 1] < key {
            return Ok(mid_idx);
         }

         if self.keys[mid_idx] > key {
            end = mid - 1;
         } else {
            start = mid + 1;
         }
      }

      panic!("Unable to find value {}", key)
   }

   /// pub fn split_node(&mut self) -> (usize, Node, Node) {
   ///    let key_len = self.keys.len();
   ///    let child_len = self.children.len();
   ///    let mid_key_idx = (key_len / 2) + 1;
   ///    let mid_key = self.get_key(mid_key_idx);
   ///
   ///    let mut right_keys = Vec::with_capacity(self.order - 1);
   ///    for i in (mid_key + 1)..key_len {
   ///       let key = self.keys.pop().unwrap();
   ///       right_keys.push(key);
   ///    }
   ///
   ///    let mut right_children = Vec::with_capacity(self.order);
   ///    for i in ((mid_key + 1)..child_len).rev() {
   ///       let node = self.children.pop().unwrap();
   ///       right_children.push(node);
   ///    }
   ///
   ///    let right_node = Node::with_vectors(right_keys, right_children, self.order, self.is_leaf, self.is_root);
   ///
   ///    (mid_key, self)
   /// }

   pub fn get_child(&self, index: usize) -> Option<&NodeRef> {
      match &self.children[index] {
         None => None,
         Some(child) => Some(&child)
      }
   }

   pub fn has_full_keys(&self) -> bool { self.keys.len() ==  self.order - 1 }

   pub fn is_root(&self) -> bool { self.parent.is_none() }
}

#[cfg(test)]
mod tests {
   use crate::node::Node;

   #[test]
   fn find_key_in_1_element() {
      let mut node = Node::new(5);
      node.keys.push(5);

      let res = node.find_key(5);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 0);

      let res = node.find_key(3);
      assert!(res.is_none());
   }

   #[test]
   fn find_key_in_2_element() {
      let mut node = Node::new(5);
      node.keys.push(5);
      node.keys.push(7);

      let res = node.find_key(5);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 0);

      let res = node.find_key(7);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 1);

      let res = node.find_key(3);
      assert!(res.is_none());

      let res = node.find_key(6);
      assert!(res.is_none());

      let res = node.find_key(8);
      assert!(res.is_none());
   }

   #[test]
   fn find_key_in_3_element() {
      let mut node = Node::new(8);
      node.keys = vec![5,7,9];

      let res = node.find_key(5);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 0);

      let res = node.find_key(7);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 1);

      let res = node.find_key(9);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 2);

      let res = node.find_key(3);
      assert!(res.is_none());

      let res = node.find_key(6);
      assert!(res.is_none());

      let res = node.find_key(8);
      assert!(res.is_none());

      let res = node.find_key(10);
      assert!(res.is_none());
   }

   #[test]
   fn find_key_in_4_element() {
      let mut node = Node::new(8);
      node.keys = vec![5,7,9,11];

      let res = node.find_key(5);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 0);

      let res = node.find_key(7);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 1);

      let res = node.find_key(9);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 2);

      let res = node.find_key(11);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 3);

      let res = node.find_key(3);
      assert!(res.is_none());

      let res = node.find_key(6);
      assert!(res.is_none());

      let res = node.find_key(8);
      assert!(res.is_none());

      let res = node.find_key(10);
      assert!(res.is_none());

      let res = node.find_key(12);
      assert!(res.is_none());
   }

   #[test]
   fn find_location_in_even_vector() {
      let mut node = Node::new(5);
      node.keys = vec![5, 10, 15, 20];

      match node.find_future_key_index(3) {
         Ok(index) => assert_eq!(index, 0, "Value must be 0 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(8) {
         Ok(index) => assert_eq!(index, 1, "Value must be 1 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(11) {
         Ok(index) => assert_eq!(index, 2, "Value must be 2 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(18) {
         Ok(index) => assert_eq!(index, 3, "Value must be 3 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(25) {
         Ok(index) => assert_eq!(index, 4, "Value must be 4 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }
   }

   #[test]
   fn find_location_in_odd_vector() {
      let mut node = Node::new(5);
      node.keys = vec![5, 10, 15, 20, 25];

      match node.find_future_key_index(3) {
         Ok(index) => assert_eq!(index, 0, "Value must be 0 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(8) {
         Ok(index) => assert_eq!(index, 1, "Value must be 1 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(11) {
         Ok(index) => assert_eq!(index, 2, "Value must be 2 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(18) {
         Ok(index) => assert_eq!(index, 3, "Value must be 3 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(23) {
         Ok(index) => assert_eq!(index, 4, "Value must be 4 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(26) {
         Ok(index) => assert_eq!(index, 5, "Value must be 5 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }
   }

   #[test]
   fn find_location_in_single_element() {
      let mut node = Node::new(5);
      node.keys = vec![5];

      match node.find_future_key_index(3) {
         Ok(index) => assert_eq!(index, 0, "Value must be 0 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }

      match node.find_future_key_index(8) {
         Ok(index) => assert_eq!(index, 1, "Value must be 1 instead got {}", index),
         Err(_) => assert!(false, "Value")
      }
   }
}