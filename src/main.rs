#![allow(dead_code)]
#![allow(unused_variables)]

const NODE_DEGREE: usize = 2;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct BTreeRules {
    maxkeys: usize,
    maxchildren: usize,
    minkeys: usize,
    minchildren: usize,
}
impl BTreeRules {
    // t = branching factor, where t >= 2
    // node must have at least t-1 keys (and t children  if not a leaf)
    // node can  have at most 2t-1 keys (and 2t children if not a leaf)
    fn new(degree: usize) -> Self {
        BTreeRules {
            // For inserts
            maxkeys: (2 * degree) - 1,
            maxchildren: 2 * degree,
            // For deletes
            minkeys: degree - 1,
            minchildren: degree,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct BTree<T, E> {
    root: Box<Node<T, E>>,
}

impl<T, E> BTree<T, E>
where
    T: std::fmt::Debug + std::cmp::Ord + Clone,
    E: std::fmt::Debug + std::cmp::Ord + Clone,
{
    fn new(degree: usize) -> Self {
        BTree {
            root: Box::new(Node::new(degree)),
        }
    }
    // at Btree-level, we want to search whole tree and only care about true/false
    fn find(&self, item: Item<T, E>) -> (usize, bool) {
        let (mut position, mut found) = self.root.search(&item);

        let mut children = &self.root.children;

        while !found && children[position].num_children > 0 {
            let node = &children[position];
            (position, found) = node.search(&item);
            children = &node.children;
        }

        (position, found)
    }
    fn root_split(&mut self) {
        println!("triggered root split");
        let (median, right_child) = self.root.split();
        let mut new_root_node: Node<T, E> = Node::new(NODE_DEGREE);
        new_root_node.items = vec![self.root.items[median].clone()];
        new_root_node.children = vec![self.root.clone(), Box::new(right_child)];
        new_root_node.num_children = 2;
        new_root_node.num_items = 1;
    }
    fn insert(&mut self, item: Item<T, E>) {
        if self.root.insert(item.clone()) {
            if self.root.rules.maxchildren <= self.root.num_items {
                self.root_split();
            }
        } else {
            println!("{:?} already exists, overwriting ...", item.key);
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct Item<T, E> {
    key: T,
    value: E,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Node<T, E> {
    items: Vec<Item<T, E>>,
    children: Vec<Box<Node<T, E>>>,
    num_items: usize,
    num_children: usize,
    rules: BTreeRules,
}

impl<T, E> Node<T, E>
where
    T: Ord + std::fmt::Debug + Clone,
    E: Ord + std::fmt::Debug + Clone,
{
    fn new(degree: usize) -> Self {
        let rules = BTreeRules::new(degree);

        Node {
            items: Vec::with_capacity(rules.maxkeys),
            children: Vec::with_capacity(rules.maxchildren),
            num_items: 0,
            num_children: 0,
            rules: BTreeRules::new(degree),
        }
    }

    fn search(&self, item: &Item<T, E>) -> (usize, bool) {
        // If key is GT Node.`items` array, return index + 1 than bounds of array
        // If key is LT Node.`items` array, return 0.
        // `true` means index returned is interpereted as the key in Node.`items.keys`
        // `false` is an index into Node.`children` array
        let mut low = 0;
        let mut high = self.items.len();
        while low < high {
            let median = (low + high) / 2;
            match item.key.cmp(&self.items[median].key) {
                std::cmp::Ordering::Less => {
                    high = median;
                }
                std::cmp::Ordering::Equal => return (median, true),
                std::cmp::Ordering::Greater => {
                    low = median + 1;
                }
            }
        }
        return (low, false);
    }
    fn insert(&mut self, item: Item<T, E>) -> bool {
        // splitting echoes throughout the tree; we try to be proactive, splitting while we visit down
        // the tree. we insert and leave, meaning we don't check if the insertion triggers a split.
        // we deal with that as the next insert's problem.
        //
        // thus, if the current node in the `insert` call is root, we'll deal with it's split in the `BTree.insert()` call.
        //
        // previously, i had `BTree.find()` as a tree `.search()`, that returned a Vec of the
        // path of indices we took down the tree to find where a key was or where it ought to be
        // inserted. i thought i could `.enumerate()` loop through that Vec to get `(depth, index)`, and on insertion, check if that required a
        // split, and then somehow run a `while` at that innermost loop level to walk back up the
        // tree performing splitting/reshuffling using `depth`. this way is cleaner and easier to reason about.

        // case 1: found item in node, overwrite and exit
        let (mut position, found) = self.search(&item);
        if found {
            self.items[position] = item;
            return false;
        }
        // case 2: you're at a leaf and it has capacity
        if self.insertable_leaf() {
            return self.insert_item(position, item);
        }
        // case 3: on your way down, if you see a full child and your node has < `maxchildren`, perform split
        if self.splittable_child(position) {
            // isn't `split` a mutable borrow during the immutable borrow by
            // `self.children[position]`?
            let (median, new_node) = self.children[position].split();
            self.children.insert(position + 1, Box::new(new_node));
            self.num_children += 1;
            // `position` is the index direction we're headed
            let new_key_to_these_split_children = self.children[position].items[median].clone();
            self.items.insert(position, new_key_to_these_split_children);
            // Now, also, we could change direction after split:
            // - say we have 1 root, 2 children, inserting key = 4
            // - after we `node-[2].search()`, we get position` = 1 (GT/right) root to right-node
            //
            //              [2]
            //              / \
            //          [0,1] [3, 8, 47]
            //
            // - our proactive split of child node at that position returns median 8
            //
            //              [ 2, 8]
            //              /  |  \
            //          [0,1] [3] [47]
            //
            // - keeping `position` = 1, at our recursive call site, will we head to right node?
            // - we check if key = 4 is GT/LT 8. It's LT, so we `position`  works.
            // - if key = 24, it would be GT, so we'd increase `position` by 1
            match item.key.cmp(&self.items[position].key) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    // due to the split, we bubbled up our match/overwrite one recursive call
                    // early.
                    self.items[position] = item;
                    return false;
                }
                std::cmp::Ordering::Greater => {
                    position += 1;
                }
            }
        }

        // try the child it should be in
        return self.children[position].insert(item);
    }
    fn splittable_child(&self, position: usize) -> bool {
        return self.children[position].num_items == self.rules.maxkeys
            && self.children.len() < self.rules.maxchildren;
    }
    fn insertable_leaf(&self) -> bool {
        return self.num_children == 0 && self.num_items < self.rules.maxkeys;
    }
    fn insert_item(&mut self, position: usize, item: Item<T, E>) -> bool {
        self.items.insert(position, item);
        self.num_items += 1;
        true
    }
    fn split(&mut self) -> (usize, Node<T, E>) {
        let mut new_node = Node::new(NODE_DEGREE);

        let median = self.items.len() / 2;

        // set new node
        new_node.items = self.items[median..].to_vec();
        new_node.num_items -= new_node.items.len();
        // update old node
        self.items = self.items[..median].to_vec();
        self.num_items -= new_node.items.len();
        (median, new_node)
    }
    fn delete() {}
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_key_simple() {
        let item_to_find = Item {
            key: 3,
            value: "horst's tavern bread",
        };
        let btree = BTree {
            root: Box::new(Node::new(NODE_DEGREE)),
        };
        let key = btree.find(item_to_find);
        assert_eq!(key, (1, true));
    }
}
//    #[test]
//    fn find_no_key_returns_false_and_position_simple() {
//        let rules = BTreeRules::new(2);
//
//        let btree = BTree {
//            root: Box::new(Node {
//                keys: vec![4, 5, 7],
//                children: vec![],
//            }),
//            rules,
//        };
//        let key = btree.find(&3);
//
//        assert_eq!(key, (3, false));
//    }
//
//    #[test]
//    fn find_no_key_returns_false_and_position_deep() {
//        let rules = BTreeRules {
//            maxkeys: 4,
//            maxchildren: 5,
//        };
//        let n1 = Box::new(Node {
//            keys: vec![1, 3, 5],
//            children: None,
//            rules: rules.clone(),
//        });
//        let n2 = Box::new(Node {
//            keys: vec![21, 42, 73],
//            children: None,
//            rules: rules.clone(),
//        });
//
//        let btree = BTree {
//            root: Box::new(Node {
//                keys: vec![7],
//                children: Some(vec![n1, n2]),
//                rules,
//            }),
//        };
//
//        let key = btree.search(81);
//        assert_eq!(key, (3, false));
//    }
//
//    #[test]
//    fn find_key_deep() {
//        let rules = BTreeRules {
//            maxkeys: 4,
//            maxchildren: 5,
//        };
//        let n1 = Box::new(Node {
//            keys: vec![1, 3, 5],
//            children: None,
//            rules: rules.clone(),
//        });
//        let n2 = Box::new(Node {
//            keys: vec![21, 42, 73],
//            children: None,
//            rules: rules.clone(),
//        });
//        let btree = BTree {
//            root: Box::new(Node {
//                keys: vec![7],
//                children: Some(vec![n1, n2]),
//                rules,
//            }),
//        };
//        let key = btree.search(42);
//        assert_eq!(key, (1, true));
//    }
//}

// every Node has left (lt) / right (gt) child node
// start at root, evaluate all the way down
// degree = maximum child nodes per parent
// a binary search tree is deep. one key per node.
// a btree does more comparisons, but less deep.
// node-fetching (Memory) takes more time
// comparing (CPU) takes less time
// the processor is waiting for the new node to be fetched (descended to)
// Root Node, Node, Leaves

// Rules:
// 1. leaves (nodes that don't point to any other nodes) all at same level. this is "balanced".
// 2. Every node except root must have min keys. > max keys, it splits.
// 3. Only time a tree gets taller is when a split causes tbe parent key to fill up and split as
//    well.
//    -- each child split pushes up the middle key to the parent
//
// 4. Deletes:
//
//  A.) if you delete from a leaf that node is now under minimum..
//
//  Take from Sibling!
//  - check child node to right of it (gt), grab left-most key, swap that with right-most
//  (separator) key in both child's parent, pull that parent key into the child node we deleted
//  from
//
//  Can't take from Sibling!
//  - merge siblings, pulling separator key from parent into merge
//  - if too few now in parent, then recursively apply this from start of step 4A.
//
//  B.) if you delete a key from middle of tree (which was acting as separtor)
//
//  - Take from Child, ensuring gt/lt rules match from new separator to the children
//  - if your take from child results in too few, use 4A section to deal with it.
//
