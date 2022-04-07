use btree_rust::BTree;

fn main() {
    let mut tree = BTree::new(3);
    let _ = tree.add(0);
    let _ = tree.add(5);
    let _ = tree.add(10);
    let _ = tree.add(15);
    let _ = tree.add(1);

    let _ = tree.delete(10);
    let _ = tree.delete(15);

    println!("Hello, world!");
}
