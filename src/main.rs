use btree_rust::BTree;

fn main() {
    let mut t = BTree::new(3);
    let _ = t.add(1);
    let _ = t.add(2);
    let _ = t.add(3);
    let _ = t.add(4);
    let _ = t.add(5);
    
    println!("Hello, world!");
}
