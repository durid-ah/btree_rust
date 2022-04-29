use btree_rust::BTree;

fn main() {
    let mut tree = BTree::new(4);
    let _ = tree.add(0);
    let _ = tree.add(5);
    let _ = tree.add(10);
    let _ = tree.add(15);
    let _ = tree.add(20);
    let _ = tree.add(25);
    let _ = tree.add(30);
    let _ = tree.add(35);
    let _ = tree.add(40);
    let _ = tree.add(45);
    let _ = tree.add(31);
    let _ = tree.add(32);

    let _ = tree.delete(35);

    println!("Hello, world!");
}
