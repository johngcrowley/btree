use std::cmp::Ordering;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct BTree<T: std::fmt::Debug + std::cmp::Ord> {
    root: Box<Node<T>>,
}

impl<T: std::fmt::Debug + std::cmp::Ord> BTree<T> {
    fn new(degree: usize) -> Self {
        BTree {
            root: Box::new(Node {
                keys: vec![],
                children: None,
                rules: BTreeRules {
                    maxkeys: (2 * degree) - 1,
                    maxchildren: 2 * degree,
                },
            }),
        }
    }
    fn search(&self, key: T) -> (usize, bool) {
        let (mut position, mut found) = self.root.search(&key);

        let mut children = &self.root.children;

        while !found && children.is_some() {
            let node = &children.as_ref().unwrap()[position];
            (position, found) = node.search(&key);
            children = &node.children;
        }

        (position, found)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct BTreeRules {
    maxkeys: usize,
    maxchildren: usize,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Node<T: Ord> {
    keys: Vec<T>,
    children: Option<Vec<Box<Node<T>>>>,
    // global check for node degree (min/max keys per)
    rules: BTreeRules,
}

/// If key is GT Node.`keys` array, return index + 1 than bounds of array
/// If key is LT Node.`keys` array, return 0.
/// `true` means index returned is interpereted as the key in Node.`keys`
/// `false` is an index into Node.`children` array, which is 1 item more than Node.`keys`.
impl<T: Ord + std::fmt::Debug> Node<T> {
    fn search(&self, key: &T) -> (usize, bool) {
        let mut low = 0;
        let mut high = self.keys.len();
        while low < high {
            let median = (low + high) / 2;
            match key.cmp(&self.keys[median]) {
                Ordering::Less => {
                    high = median;
                }
                Ordering::Equal => return (median, true),
                Ordering::Greater => {
                    low = median + 1;
                }
            }
        }
        return (low, false);
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn find_key_simple() {
        let btree = BTree {
            root: Box::new(Node {
                keys: vec![2, 3, 4, 5],
                children: None,
                rules: BTreeRules {
                    maxkeys: 4,
                    maxchildren: 5,
                },
            }),
        };
        let key = btree.search(3);
        println!("key: {:?}", key);
        assert_eq!(key, (1, true));
    }
    #[test]
    fn find_no_key_returns_false_and_position_simple() {
        let rules = BTreeRules {
            maxkeys: 4,
            maxchildren: 5,
        };
        let btree = BTree {
            root: Box::new(Node {
                keys: vec![4, 5, 7],
                children: None,
                rules,
            }),
        };

        let key = btree.search(8);
        assert_eq!(key, (3, false));
    }

    #[test]
    fn find_no_key_returns_false_and_position_deep() {
        let rules = BTreeRules {
            maxkeys: 4,
            maxchildren: 5,
        };
        let n1 = Box::new(Node {
            keys: vec![1, 3, 5],
            children: None,
            rules: rules.clone(),
        });
        let n2 = Box::new(Node {
            keys: vec![21, 42, 73],
            children: None,
            rules: rules.clone(),
        });

        let btree = BTree {
            root: Box::new(Node {
                keys: vec![7],
                children: Some(vec![n1, n2]),
                rules,
            }),
        };

        let key = btree.search(81);
        assert_eq!(key, (3, false));
    }

    #[test]
    fn find_key_deep() {
        let rules = BTreeRules {
            maxkeys: 4,
            maxchildren: 5,
        };
        let n1 = Box::new(Node {
            keys: vec![1, 3, 5],
            children: None,
            rules: rules.clone(),
        });
        let n2 = Box::new(Node {
            keys: vec![21, 42, 73],
            children: None,
            rules: rules.clone(),
        });
        let btree = BTree {
            root: Box::new(Node {
                keys: vec![7],
                children: Some(vec![n1, n2]),
                rules,
            }),
        };
        let key = btree.search(42);
        assert_eq!(key, (1, true));
    }
}

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
