# btree
# btree in Rust

An in-memory example of the BTree algorithim in Rust, using a proactive, single, downward pass

```
cargo test
```

---

# ToDo:
- [ ] Convert Node types to `enum` with different methods for `Node::Internal` versus `Node::Leaf`
- [ ] Handle a root key delete
- [ ] Zero-copy wherever possible
- [ ] Tests for different data types
- [ ] Move from in-memory to file system
- [ ] Make `print()` prettier or at least more clear
