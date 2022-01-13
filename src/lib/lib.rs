use node::{Node, NodeRef};

mod node;

pub struct BTree {
    order: usize,
    root: NodeRef
}