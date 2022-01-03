use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    keys: Vec<usize>,
    children: Vec<Rc<RefCell<Node>>>
}

impl Node {
    pub fn new(order: usize) -> Self {
        return Self{
            keys: Vec::with_capacity(order - 1),
            children:Vec::with_capacity(order)
        }
    }

    pub fn add_key(&mut self, key: usize) {

        self.keys.push(key);
        let mut new_key_idx = self.keys.len() - 1;

        if new_key_idx == 0 {
            return;
        }

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
        let mut start = 0;
        let mut end = self.keys.len();



        while end >= start {
            let mut mid = start + (end - start) / 2;

            if self.keys[mid] == key {
               return Option::Some(mid);
            }

            if self.keys[mid] > key {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        return Option::None;
    }

    pub fn get_key(&self, index: usize) -> usize {
        return self.keys[index];
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

        let mut node = Node::new(5);

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
}