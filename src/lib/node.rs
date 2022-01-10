
pub struct Node {
   pub is_root: bool,
   pub is_leaf: bool,
   order: usize,
   min_child_count: usize,
   keys: Vec<usize>,
   children: Vec<Node>
}

impl Node {
   pub fn new(order: usize, is_root: bool, is_leaf: bool) -> Self {
      let min_child_count: usize = order / 2;

      return Self{
         keys: Vec::with_capacity(order - 1),
         children:Vec::with_capacity(order),
         order,
         min_child_count,
         is_leaf,
         is_root
      }
   }

   pub fn with_vectors(
      keys: Vec<usize>, children: Vec<Node>, order: usize, is_leaf: bool, is_root: bool) -> Node {

      let min_child_count: usize = order / 2;
      if keys.len() > order - 1 {
         panic!("There are too many keys");
      }

      if children.len() > order || children.len() < min_child_count {
         panic!("There are too many children");
      }

      return Node {keys, children, order, is_root, is_leaf, min_child_count}
   }

   pub fn add_key(&mut self, key: usize) {
      // if the value already exists just update the location
      if let Option::Some(idx) = self.find_key(key) {
         self.keys[idx] = key;
         return;
      }

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

   pub fn split_node(&mut self) -> Node {
      let key_len = self.keys.len();
      let child_len = self.children.len();
      let mid_key_idx = (key_len / 2) + 1;
      let mid_key = self.get_key(mid_key_idx);

      let mut new_parent = Node::new(self.order, self.is_root, false);
      new_parent.add_key(mid_key);

      let mut right_keys = Vec::with_capacity(self.order);
      for i in (mid_key + 1)..key_len {
         right_keys.push(self.keys[i]);
      }

      for i in (mid_key + 1)..key_len {
         right_keys.pop();
      }

      let mut right_children = Vec::with_capacity(self.order);
      for i in ((mid_key + 1)..child_len).rev() {
         let node = self.children.pop().unwrap();
         right_children.push(node);
      }

      new_parent
   }

   pub fn get_key(&self, index: usize) -> usize {
      return self.keys[index];
   }

   pub fn get_child(&self, index: usize) -> &Node { return  &self.children[index]; }

   pub fn has_full_keys(&self) -> bool { self.keys.len() ==  self.order - 1 }

   pub fn has_full_children(&self) -> bool { self.children.len() == self.order }

   pub fn has_min_children(&self) -> bool {
      if self.is_root {
         return self.children.len() == 2;
      }

      self.children.len() == self.min_child_count
   }
}

#[cfg(test)]
mod tests {
   use crate::node::Node;

   #[test]
   fn adding_method() {
      const FIRST: usize= 16;
      const SECOND: usize = 10;
      const THIRD: usize = 20;
      const FOURTH: usize = 18;
      const FIFTH: usize = 25;

      let mut node = Node::new(5, true, true);

      node.add_key(FIRST);
      assert_eq!(node.get_key(0), FIRST);

      node.add_key(SECOND);
      assert_eq!(node.get_key(0), SECOND);
      assert_eq!(node.get_key(1), FIRST);

      node.add_key(THIRD);
      assert_eq!(node.get_key(0), SECOND);
      assert_eq!(node.get_key(1), FIRST);
      assert_eq!(node.get_key(2), THIRD);

      node.add_key(FOURTH);
      assert_eq!(node.get_key(0), SECOND);
      assert_eq!(node.get_key(1), FIRST);
      assert_eq!(node.get_key(2), FOURTH);
      assert_eq!(node.get_key(3), THIRD);

      node.add_key(FIFTH);
      assert_eq!(node.get_key(0), SECOND);
      assert_eq!(node.get_key(1), FIRST);
      assert_eq!(node.get_key(2), FOURTH);
      assert_eq!(node.get_key(3), THIRD);
      assert_eq!(node.get_key(4), FIFTH);
   }

   #[test]
   fn find_key_in_1_element() {
      let mut node = Node::new(5, true, true);
      node.add_key(5);

      let res = node.find_key(5);
      assert!(res.is_some());
      assert_eq!(res.unwrap(), 0);

      let res = node.find_key(3);
      assert!(res.is_none());
   }

   #[test]
   fn find_key_in_2_element() {
      let mut node = Node::new(5, true, true);
      node.add_key(5);
      node.add_key(7);

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
      let mut node = Node::new(8, true, true);
      node.add_key(5);
      node.add_key(7);
      node.add_key(9);

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
      let mut node = Node::new(8, true, true);
      node.add_key(5);
      node.add_key(7);
      node.add_key(9);
      node.add_key(11);


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
}