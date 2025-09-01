#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::BTreeMap;
use std::fmt::{Display, Debug};
use std::cmp::Ordering;

const NODE_DEGREE: usize = 2;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct BTreeRules {
    maxkeys: usize,
    maxchildren: usize,
    minkeys: usize,
    minchildren: usize,
    degree: usize,
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
            degree,
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct BTree<T, E> {
    root: Box<Node<T, E>>,
}

impl<T, E> BTree<T, E>
where
    T: Debug + Ord + Clone + Display,
    E: Debug + Ord + Clone + Display,
{
    fn new(degree: usize) -> Self {
        BTree {
            root: Box::new(Node::new(degree)),
        }
    }
    
    fn print(&self) {
        
        // travel the tree, filling up BTreeMap vec 
        let depth = 1;
        let mut tree: BTreeMap<i32, Vec<String>> = BTreeMap::new();
        let node = &self.root;
        
        fn descend_printer<'a,T: Debug + Display,E: Debug>(depth: i32, treemap: &mut BTreeMap<i32, Vec<String>>, node: &'a Node<T,E>) {
            let next_items = &node.items;
            treemap.entry(depth).or_insert(Vec::new()).push(
                format!("[{}]",
                    next_items
                        .iter()
                        .map(|node| {
                           format!("{}", node.key) 
                        }).collect::<Vec<String>>().join(",")
                )
            );
            if node.children.len() > 0 {
                for node in &node.children {
                    descend_printer(depth+1, treemap, &node);
                }
            } else {
                return
            }
        }

        // recursively load map
        descend_printer(depth, &mut tree, node);
        
        // gather formatting 
        let formatted: BTreeMap<i32, String> = tree
            .into_iter()
            .map(|(depth_, nodes)| {
                let mut depth_space = "".to_string();
                for _ in 0..10/depth_ {
                    depth_space += "   ";
                }
                (
                    depth_, nodes
                                .into_iter()
                                .collect::<Vec<String>>().join(&depth_space)
                ) 
            }).collect();

        // print formatted btree 
        let max_depth = formatted.keys().last().unwrap();
        let max_depth_len = formatted.get(max_depth).unwrap().len() as i32;
        for (tree_depth, nodes_at_depth) in formatted.iter() {
            
            let indent = max_depth_len / (tree_depth + (1));
            let mut prefix_space = "".to_string();
            for _ in 0..indent {
                prefix_space += " ";
            }
            println!("{prefix_space}{nodes_at_depth}");
            println!("");
        }
    }
    
    fn find(&self, item: Item<T, E>) -> (usize, bool) {
        let (mut position, mut found) = self.root.binary_search(&item);
        // descend only if kids, else index out of bounds
        if self.root.num_children > 0 {
            let mut children = &self.root.children;
            while !found && children.len() > 0 {
                let node = &children[position];
                (position, found) = node.binary_search(&item);
                children = &node.children;
            }
        }
        (position, found)
    }
    fn root_split(&mut self) {
        
        println!("triggered root split");
        let (median, right_child) = self.root.split();
        self.root = Box::new(
            Node {
                items: vec![median],
                children: vec![self.root.clone(), Box::new(right_child)],
                num_items: 1,
                num_children: 2,
                rules: BTreeRules::new(NODE_DEGREE),
            }
        );
    }
    fn insert(&mut self, item: Item<T, E>) {
        let key = item.key.clone();
        if self.root.num_items >= self.root.rules.maxkeys {
            self.root_split();
        }
        if self.root.insert(item) {
            println!("inserted key {key} into tree ...");
        } else {
            println!("{key} already exists, overwriting ...");
        }
    }
    fn delete(&mut self, item: Item<T, E>) {
        
        // case 0: this is the root and we lower the height of the tree
        let key = item.key.clone();
        let (_, found) = self.root.binary_search(&item);
        if found {
            println!("deal with this at some point");
        } else {
            let output = self.root.delete(item);
            println!("deleted item with key: {} from btree", key);
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
    T: Ord + Debug + Clone,
    E: Ord + Debug + Clone,
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

    fn binary_search(&self, item: &Item<T, E>) -> (usize, bool) {
        // If key is GT Node.`items` array, return index + 1 than bounds of array
        // If key is LT Node.`items` array, return 0.
        // `true` means index returned is interpereted as the key in Node.`items.keys`
        // `false` is an index into Node.`children` array
        let mut low = 0;
        let mut high = self.items.len();
        while low < high {
            let median = (low + high) / 2;
            match item.key.cmp(&self.items[median].key) {
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
    fn merge(&mut self, position: usize, sibling: usize) {
        
       let push_down_key = self.items.remove(position);
       self.num_items -= 1;
       let mut node_1 = self.children[position].clone();
       let node_2 = self.children[sibling].clone();
       
       println!("we had to merge these two nodes on our descent:\n\t{:?}\n\t{:?}", node_1, node_2);
       
       // copy first
       let num_items = node_1.items.len() + node_2.items.len() + 1;
       let num_children = node_1.children.len() + node_2.children.len();
       
       // then moves
       node_1.children.extend(node_2.children);
       node_1.items.push(push_down_key);
       node_1.items.extend(node_2.items);
       
       // merged child reducing children count
       self.children[position] = Box::new(
           Node {
               items: node_1.items,
               children: node_1.children,
               num_items,
               num_children,
               rules: BTreeRules::new(NODE_DEGREE),
           }
       );
       self.children.remove(sibling);
       self.num_children -= 1;
    }
    fn swap(&mut self, mut position: usize, sibling: usize) -> usize {
        
       println!("we had to swap keys");

       let mut siblings_child_pointer_idx = 0;
       let pushed_parent_key = self.items[position].clone();
           
       match position.cmp(&sibling) {
           Ordering::Greater => {
              // bring over sibling key
              let rightmost = self.children[sibling].items.pop().unwrap();
              // update child key with parent's
              self.children[position].items.insert(0, pushed_parent_key);
              // update parent key with sibling's
              self.items[position] = rightmost;
              // store this for shuffling children -- here, its one idx gt len (0-indexed),
              // meaning, furthest-right child pointer
              siblings_child_pointer_idx = self.children[sibling].items.len();
           },
           Ordering::Less => {
              // bring over sibling key
              let leftmost = self.children[sibling].items.remove(0);
              // update child key with parent's
              self.children[position].items.push(pushed_parent_key.clone());
              // update parent key with sibling's
              self.items[position] = leftmost;
           },
           _=> {}
       }
       self.children[position].num_items += 1;
       
       // pull over sibling's relative child keys, if any  
       if !self.children[position].leaf() {
           let child_swap = self.children[sibling].children.remove(siblings_child_pointer_idx);
           if siblings_child_pointer_idx == 0 {
               self.children[position].children.push(child_swap);
           } else {
               self.children[position].children.insert(0, child_swap);
           }
           self.children[position].num_children += 1;
           self.children[sibling].num_children -= 1;
       }
       position
    }
       
    fn make_enough(&mut self, mut position: usize) -> usize {
        
        //! position stays the same, not being updated in swap or merge case

        // has children. man. should make enums and slap this on `Node::Internal` as a method.
        if !self.children[position].enough() {

            let sibling: usize;
            // look left or right?
            if position == self.children.len() - 1 {
                // can only look left for help
                sibling = position-1;
            } else {
                // can only look right for help, our current default even for middle nodes
                sibling = position+1;
            }
            
            tracing::debug!("\n(x) current: \n\t{:?}\n", self.items);
            tracing::debug!("\n(y) child: \n\t{:?}\n", self.children[position].items);
            tracing::debug!("\n(z) sibling: \n\t{:?}\n", self.children[sibling].items);
            
            if self.children[sibling].enough() {
                // position may alter
                position = self.swap(position, sibling);
            }
            else {
                self.merge(position, sibling);
            }
        }
        position
    }
    
    fn delete(&mut self, item: Item<T, E>) -> Option<Item<T, E>> {
        
        /*! 
          - Do not descend unless enough keys 
          - KTD = key to delete
        !*/

        // A1.i. look for item to delete
        let (position, found) = self.binary_search(&item);
        
        // A1.ii. base case: intent is fetching a path order-preserving max key for an internal node delete up call stack
        if !found && self.leaf() {
            return self.items.pop();
        }

        // A2: only descend if there is enough in next node in recursion path
        if !found {
            self.make_enough(position);
        }

        // A1.iii. base case: plain old goodbye
        if found && self.leaf() {
            self.items.remove(position);
            self.num_items -= 1;
            return None;
        }

        // Recursion call site, but also another base case:
        // A1.iv. base case: internal node needs to go get the deepest, biggest key (arbitrarily
        //                   leftwards), it can find to preserve order when it removes its own
        if let Some(key) = self.children[position].delete(item) {
            // we know we are back at internal node if `found`, and can end our run.
            if found {
               self.items[position] = key;
               return None
            } 
            // keep passing it up the callstack
            return Some(key)
        }

        None
    }
    fn insert(&mut self, item: Item<T, E>) -> bool {
        // splitting echoes throughout the tree. we try to be proactive, splitting-while-visit
        // in one downward pass. we insert and leave, meaning we don't check if the insertion triggers a split.
        // we deal with that as the next insert's problem.
        // thus, if the current node in the `insert` call is root, we'll deal with it's split in the `BTree.insert()` call.

        // case 1: found item in node, overwrite and exit
        let (mut position, found) = self.binary_search(&item);
        if found {
            self.items[position] = item;
            return false;
        }
        // case 2: you're at a leaf and it has capacity
        if self.insertable() && self.leaf() {
            self.items.insert(position, item);
            self.num_items += 1;
            return true;
        }
        // case 3: on your way down, if you see a full child, split.
        if self.splittable_child(position) {
            // isn't `split` a mutable borrow during the immutable borrow by
            // `self.children[position]`?
            let (median, new_node) = self.children[position].split();

            self.children.insert(position + 1, Box::new(new_node));
            self.num_children += 1;
            // `position` is the index direction we're headed down, 
            self.items.insert(position, median);
            self.num_items += 1;
           // change recursive path in case a split brought up a median into our items making
           // `position` outdated
           if &item.key > &self.items[position].key {
               position += 1;
           }
        }
        return self.children[position].insert(item);
    }
    fn splittable_child(&self, position: usize) -> bool {
        return self.children[position].num_items == self.rules.maxkeys
            && self.children.len() < self.rules.maxchildren;
    }
    fn leaf(&self) -> bool {
        return self.num_children == 0;
    }
    fn insertable(&self) -> bool {
        return self.num_items < self.rules.maxkeys;
    }
    fn enough(&self) -> bool {
       return self.num_items >= self.rules.degree
    }
    fn split(&mut self) -> (Item<T, E>, Node<T, E>) {
        let mut new_node = Node::new(NODE_DEGREE);

        // -- split the items
        let median = self.items.len() / 2;
         
        // additional node
        new_node.items = self.items[median+1..].to_vec();
        new_node.num_items = new_node.items.len();
        
        // first node
        let new_items = self.items[..median].to_vec();
        self.num_items = new_items.len();    
        
        // now, extract median Item to pass up to parent
        let median_item = self.items.remove(median);
        self.items = new_items;
      
        // -- split the children
        let children_median = self.children.len() / 2;
           
        // additional node
        new_node.children = self.children[children_median..].to_vec();
        new_node.num_children = new_node.children.len();
         
        // first node  
        self.children = self.children[..children_median].to_vec();
        self.num_children -= new_node.num_children;
            
        (median_item, new_node)
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_test_tree() -> BTree<i32, &'static str> {
       
        let items = vec![
            Item {
                key: 23,
                value: "Nerevar's Ring",
            },
            Item {
                key: 67,
                value: "Vivec's Tears",
            },
            Item {
                key: 89,
                value: "Dwemer Cogwheel",
            },
            Item {
                key: 45,
                value: "Telvanni Bug Musk",
            },
            Item {
                key: 78,
                value: "Kagrenac's Tools",
            },
            Item {
                key: 34,
                value: "Moon Sugar",
            },
            Item {
                key: 91,
                value: "Almalexia's Grace",
            },
            Item {
                key: 56,
                value: "Cliff Racer Plume",
            },
            Item {
                key: 16,
                value: "Nerevarine's Gauntlet",
            },
            Item {
                key: 47,
                value: "Dunmer Ancestor Silk",
            },
            Item {
                key: 81,
                value: "Red Mountain Ash",
            },
        ];

        // root
        let mut root = Node::new(NODE_DEGREE);
        root.items = vec![Item {
            key: 7,
            value: "Daedric Bow",
        }];
        root.num_items += 1;

        // btree
        let mut btree = BTree {
            root: Box::new(root)
        };

        // insert
        for item in items {
            btree.insert(item);
        }
        
        // output
        btree.print();
        
        btree
    }

    #[test]
    fn find_key_simple() {

        let btree = setup_test_tree();
        
        let item_to_find = Item {
            key: 81,
            value: "Red Mountain Ash",
        };
        
        let key = btree.find(item_to_find);
        assert_eq!(key, (1, true));
    }
    #[test]
    fn delete_root() {
        
        let mut btree = setup_test_tree();
        let item_to_delete = Item {
            key: 7,
            value: "zonko's",
        };
        let output = btree.delete(item_to_delete);
    }
    #[test]
    fn delete_internal() {
        
        let mut btree = setup_test_tree();
        let item_to_delete = Item {
            key: 89,
            value: "Dwemer Cogwheel",
        };
        
        let key = btree.find(item_to_delete.clone());
        assert_eq!(key, (1, true));
        btree.print();
        
        let output = btree.delete(item_to_delete.clone());

        btree.print();
        
        let key = btree.find(item_to_delete);
        assert_eq!(key, (0, false));
        
    }
    #[test]
    fn delete_leaf() {
        
        let mut btree = setup_test_tree();
        let item_to_delete = Item {
            key: 47,
            value: "Dunmer Ancestor Silk",
        };
        let key = btree.find(item_to_delete.clone());
        assert_eq!(key, (0, true));
        btree.print();
        
        let output = btree.delete(item_to_delete.clone());

        btree.print();
        
        let key = btree.find(item_to_delete);
        assert_eq!(key, (0, false));
        
    }
    #[test]
    fn delete_leaf_at_minimum() {
        
        let mut btree = setup_test_tree();
        let item_to_delete = Item {
            key: 34,
            value: "Moon Sugar",
        };
        let key = btree.find(item_to_delete.clone());
        assert_eq!(key, (0, true));
        btree.print();
        
        let output = btree.delete(item_to_delete.clone());

        btree.print();
        
    }
}


