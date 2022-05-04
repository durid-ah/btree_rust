use crate::{Node, NodeRef};
use std::{rc::Rc, cell::{Ref, RefMut}};

pub type OpResult = Result<(),()>;

impl Node {
    pub(super) fn update_children_indexes(&mut self) {
        self.children.iter_mut()
           .enumerate()
           .for_each(|(i, c)| c.borrow_mut().index_in_parent = Some(i));
    }

    pub(super) fn borrow_child(&self, index: usize) -> Ref<'_, Node> {
        self.children[index].borrow()
    }

    pub(super) fn borrow_child_mut(&self, index: usize) -> RefMut<'_, Node> {
        self.children[index].borrow_mut()
    }

    /// Insert child node and put it into the proper order
    pub fn add_child(&mut self, child: NodeRef) {
        self.children.push(child);

        let mut new_child_idx = self.children.len() - 1;
        self.borrow_child_mut(new_child_idx).index_in_parent = Some(new_child_idx);

        // if the new child is in the first position there is no need for ordering
        if new_child_idx == 0 { return; }

        let mut current_idx = new_child_idx - 1;

        loop {
            let current_child = self.borrow_child(current_idx);
            let new_child = self.borrow_child(new_child_idx);
            let current_val = current_child.get_max_key();
            let new_child_val = new_child.get_min_key();

            // if the value is in the right spot end the loop
            if new_child_val > current_val { break; }

            drop(new_child);
            drop(current_child);
            self.children.swap(new_child_idx, current_idx);

            if current_idx > 0 {
                new_child_idx = current_idx;
                current_idx -= 1;
            }
        }

        self.update_children_indexes()
    }

    /// Return a cloned pointer to the child node at a given index
    pub fn try_clone_child(&self, index: isize) -> Option<NodeRef> {
        if self.children.is_empty() || index < 0 {
            return Option::None;
        }

        Some(Rc::clone(&self.children[index as usize]))
    }

    pub fn try_move_key_from_left_child(&mut self, index: usize) -> OpResult
    {
        self.try_move_key_from_child(index, true)
    }

    pub fn try_move_key_from_right_child(&mut self, index: usize) -> OpResult
    {
        self.try_move_key_from_child(index, false)
    }

    pub fn try_move_key_from_child(&mut self, index: usize, is_left: bool) -> OpResult
    {
        let child_ref: NodeRef = self
            .try_clone_child(index as isize).ok_or(())?;
            
        let mut child = child_ref.borrow_mut();
        let key_idx_to_move = if is_left { 0 } else { child.keys.len() };    
        if child.has_more_than_min_keys() {
            let child_key = child.keys.remove(key_idx_to_move);
            self.add_key(child_key);
            Ok(())
        } else {
            Err(())
        }
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
        assert_eq!(second.borrow_mut().get_key(0), 2);
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
