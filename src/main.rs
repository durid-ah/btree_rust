use btree_rust::BTree;

fn main() {
    let mut tree = BTree::new(3);
    let _ = tree.add(1);
    let _ = tree.add(2);
    let _ = tree.add(3);
    let _ = tree.add(4);
    let _ = tree.add(5);
    let _ = tree.add(6);
    let _ = tree.add(7);

    println!("Hello, world!");
}
