## btree in Rust

An in-memory example of the BTree algorithim in Rust, using a proactive, single, downward pass.

Once I stopped looking at various posts and videos about the "cases" for delete, and whiteboarded an internal node in a deep tree being deleted,
a lot clicked for me and it felt intuitive. I really like the recursive algoritihm using `Option<T>` to pass up the new divider key in the internal node. That was my favorite part.

```
git clone https://github.com/johngcrowley/btree.git && cd btree
cargo test
```
---

## ToDo:
- [ ] Convert Node types to `enum` with different methods for `Node::Internal` versus `Node::Leaf`
- [ ] Handle a root key delete
- [ ] Zero-copy wherever possible
- [ ] Tests for different data types
- [ ] Move from in-memory to file system
- [ ] Make `print()` prettier or at least more clear
