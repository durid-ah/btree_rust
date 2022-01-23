use std::cell::RefCell;
use std::rc::Rc;
use node::{Node, NodeRef};

mod node;

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

    pub fn add(value: usize) {

    }
}

/// Insert value
/// 1. Check if it fits in the first node (binary search style)
/// 2. Check if it fits on the child between the two keys (lager than the largest key?)
/// 3. If it
fn hello() {
    return;
}

