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
//          ---
//
// - keeping `position` = 1, at our recursive call site, will we head to right node?
// - we check if key = 4 is GT/LT 8. It's LT, so we `position`  works.
// - if key = 24, it would be GT, so we'd increase `position` by 1
//
//              [ 2, 8, "(58)"] -> split
//              /  |  \
//          [0,1] [3,5] [12,47,"(58)"] -> split
//
//    then we add a '58'.
//    half kids go with each split half
//
//                  [ 8 ]
//                  /    \
//                [2]     [58]
//              /    \    /   \
//          [0,1]  [3,5] [12] [47]
//
//  each split has to return the median value, and the new node it made
//
//  each parent of that has to receive a median and the new node, then
//  - insert medium into their self.items
//  - check if thats a split
//  - if itself doesnt split, insert received node into its children array at
//  position + 1. this works if its on L or R of any key. doesnt matter.
//  - if it splits, split, give half its childen to new node before passing it up
//
//  kids
//
// what if we had 8 as max keys?
//  we'd have 9 children per node
//  a split is a split! its always divide by 2!  

 
