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

    let res = tree.delete(35);


    // let mut tree = BTree::new(4);
    // let _ = tree.add(0);
    // let _ = tree.add(5);
    // let _ = tree.add(10);
    // let _ = tree.add(15);
    // let _ = tree.add(20);
    // let _ = tree.add(25);
    // let _ = tree.add(30);
    // let _ = tree.add(35);
    // let _ = tree.add(40);

    // let _ = tree.add(45);
    // let _ = tree.add(50);
    // let _ = tree.add(55);
    // let _ = tree.add(60);
    // let _ = tree.add(65);
    // let _ = tree.add(70);
    // let _ = tree.add(75);

    // let _ = tree.add(51);

    // let _ = tree.delete(55);

    // println!("Hello, world!");
}
