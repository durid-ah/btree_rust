use std::cell::RefCell;
use std::rc::Rc;
use node::{Node, NodeRef};

mod node;

const ALREADY_EXISTS_ERROR: &str = "Value already exists";

pub struct BTree {
    order: usize,
    root: NodeRef
}

impl BTree {
    pub fn new(order: usize) -> Self {
        return Self {
            order,
            root: Rc::new(RefCell::new(Node::new(order)))
        }
    }

    /// Add a value into the tree or return an error if the value already exists
    /// Works by searching each node for a possible location in every node
    /// until there is no child to insert it in
    pub fn add(&mut self,value: usize) -> Result<(), &str> {
        let mut node = self.find_insert_node(value);

        // TODO: Attempt to insert in the res location
        // TODO: New Method for the splitting process
        // TODO: Check for splitting
        // TODO: Add split into parent
        // TODO: Check if parent needs to split

        return Ok(());
    }

    fn find_insert_node(&mut self, value: usize) -> Result<usize, &str> {
        let mut node = self.root.as_ptr(); // TODO: change to get &NodeRef
        let mut res;
        unsafe {
            res = (*node).find_future_key_index(value);

            if res.is_err() { return Err(ALREADY_EXISTS_ERROR); }

            loop {
                let node_option = (*node).get_child(res.as_ref().unwrap());
                if node_option.is_none() {
                    break;
                }

                node = node_option.unwrap().as_ptr();
                res = (*node).find_future_key_index(value);
                if res.is_err() {
                    return Err(ALREADY_EXISTS_ERROR);
                }
            }
        }

        // TODO: Return &NodeRef instead
        return Ok(res.unwrap())
    }

    // TODO: Main Split Method:
    // TODO: Check for split
    // TODO: if not return
    // TODO: Insert key and children into parent
    // TODO: Loop again
    // TODO: See Node for split method
}

#[cfg(test)]
mod tests {
    use crate::BTree;

    #[test]
    fn test_find_node() {
        let mut tree = BTree::new(5);
        tree.find_insert_node(2);
    }
}