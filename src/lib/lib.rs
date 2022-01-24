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
        unsafe {
            let mut node = self.root.as_ptr();
            let mut res = (*node).find_future_key_index(value);

            if res.is_err() {
                return Err(ALREADY_EXISTS_ERROR);
            }

            loop {
                let node_option = (*node).get_child(res.unwrap());
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

        return Ok(());
    }
}