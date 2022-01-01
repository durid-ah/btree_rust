use std::cell::RefCell;
use std::rc::Rc;

pub struct Node {
    keys: Vec<usize>,
    children: Vec<Rc<RefCell<Node>>>
}