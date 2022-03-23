use std::cell::RefCell;
use std::rc::Rc;
use crate::{Node, NodeRef};

pub(crate) fn calculate_mid(start: isize, end: isize) -> isize { ((end - start) / 2) + start }

pub(crate) fn new_node_ref(order: usize) -> NodeRef {
   Rc::new(RefCell::new(Node::new(order)))
}
