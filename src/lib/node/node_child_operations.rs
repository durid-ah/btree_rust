use crate::{Node, NodeRef};
use std::rc::Rc;

impl Node {
    /// Insert child node and put it into the proper order
    pub fn add_child(&mut self, child: NodeRef) {
        self.children.push(child);

        let mut new_child_idx = self.children.len() - 1;
        self.children[new_child_idx].borrow_mut().index_in_parent = Some(new_child_idx);

        // if the new child is in the first position there is no need for ordering
        if new_child_idx == 0 {
            return;
        }

        let mut current_idx = new_child_idx - 1;

        loop {
            let mut current_child = self.children[current_idx].borrow_mut();
            let mut new_child = self.children[new_child_idx].borrow_mut();
            let current_val = current_child.get_max_key();
            let new_child_val = new_child.get_min_key();

            if new_child_val > current_val {
                // if the value is in the right spot end the loop
                break;
            }

            new_child.index_in_parent = Some(current_idx);
            current_child.index_in_parent = Some(new_child_idx);
            drop(new_child);
            drop(current_child);
            self.children.swap(new_child_idx, current_idx);

            if current_idx > 0 {
                new_child_idx = current_idx;
                current_idx -= 1;
            }
        }
    }

    /// Remove the child at the specified index and update the
    /// indices of the children to the left
    pub fn remove_child(&mut self, index: usize) {
        self.children.remove(index);
        for idx in index..self.children.len() {
            self.children[idx].borrow_mut().index_in_parent = Some(idx);
        }
    }

    /// Return a cloned pointer to the child node at a given index
    pub fn try_clone_child(&self, index: isize) -> Option<NodeRef> {
        if self.children.is_empty() || index < 0 {
            return Option::None;
        }

        Some(Rc::clone(&self.children[index as usize]))
    }
}

#[cfg(test)]
mod child_tests {
    use super::*;
    use std::cell::RefCell;

    fn build_parent_and_two_nodes() -> (Node, NodeRef, NodeRef) {
        let parent = Node::new(5);

        let first_child: NodeRef = Rc::new(RefCell::new(Node::new(5)));
        first_child.borrow_mut().add_key(1);

        let second_child: NodeRef = Rc::new(RefCell::new(Node::new(5)));
        second_child.borrow_mut().add_key(2);

        return (parent, first_child, second_child);
    }

    #[test]
    fn add_children_in_order() {
        let (mut parent, first_child, second_child) = build_parent_and_two_nodes();

        parent.add_child(first_child);
        parent.add_child(second_child);

        let first = parent.try_clone_child(0).unwrap();
        let second = parent.try_clone_child(1).unwrap();

        assert_eq!(first.borrow_mut().get_key(0), 1);
        assert_eq!(first.borrow_mut().index_in_parent.unwrap(), 0);
        assert_eq!(second.borrow_mut().get_key(0), 2);
        assert_eq!(second.borrow_mut().index_in_parent.unwrap(), 1);
    }

    #[test]
    fn add_children_out_of_order() {
        let (mut parent, first_child, second_child) = build_parent_and_two_nodes();

        parent.add_child(second_child);
        parent.add_child(first_child);

        let first = parent.try_clone_child(0).unwrap();
        let second = parent.try_clone_child(1).unwrap();

        assert_eq!(first.borrow_mut().get_key(0), 1);
        assert_eq!(first.borrow_mut().get_key(0), 1);
        assert_eq!(first.borrow_mut().index_in_parent.unwrap(), 0);
        assert_eq!(second.borrow_mut().get_key(0), 2);
        assert_eq!(second.borrow_mut().index_in_parent.unwrap(), 1);
    }
}
