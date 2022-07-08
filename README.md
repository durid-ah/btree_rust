# RefCell BTree
A btree built using `Rc<RefCell<T>>` that balances using pre-emptive splitting and merges. The tree requires all values to be unique

# Usage:
A tree can simply be instantiated using:
```rust
let tree = BTree::new(2); // 2: Represents the minimum degree
// Insertion:
let _ = tree.insert(1); // Ok
let _ = tree.insert(1); // Error: Value Already Exists

// Find:
let _ = tree.find(1) // Ok
let _ = tree.find(10) // Error: Value Not Found

// Deletion:
let _ = tree.delete(1) // Ok
let _ = tree.delete(1) // Error: Value Not Found
```

# Installation:
In order to use the btree library, the dependency can be added as follows in the `Cargo.toml` file
```toml
...
[dependencies]
...
ref_cell_btree = {git = "https://github.com/durid-ah/btree_rust"}
```
