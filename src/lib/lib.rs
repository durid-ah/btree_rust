use node::Node;

mod node;

pub struct BTree {
    order: usize,
    root: Node
}