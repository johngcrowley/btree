fn insert_reactive(&mut self, item: Item<T, E>) -> Option<(Item<T,E>, Node<T, E>)> {
    // binary search of node.items. either it's there, or you have the position/index of which child to check next.
    let (position, found) = self.binary_search(&item);

    // already here! overwrite key with value -- self is a leaf
    if found {
        self.items[position] = item.clone();
        // return because it's an overwrite, this won't be the straw that broke the camel's back.
        return None
    }

    // bottom case of recursion -- self is a leaf
    if self.leaf() {
        self.items.insert(position, item);
        self.num_items += 1;

        // send stuff back up a stack frame (to parent)
        if self.full_reactive() {
            // split() will insert our value
            let (median_of_split, new_child_node_we_made) = self.split();
            // `return` of this `self` is the child to parent stack frame (line #165)
            return Some((median_of_split, new_child_node_we_made));
        }
        // nothing to tell dad
        return None;
    }

    // -- self is parent
    // here we enter recursion to hit bottom case or "found". but bottom case has either (1) an "insert"
    // or (2) "split and insert", so we wait for that child's return in this stack frame.
    let insert_in_child_at_position = self.children[position].insert_reactive(item);

    // (2) split logic handled here. happens upward, so we don't have to alter the path down like we do in 'proactive'.
    if let Some((median_idx_of_split, new_child_node_we_made)) = insert_in_child_at_position {
        let new_key_to_these_split_children =
            self.children[position].items[median_idx_of_split].clone();
        // add split's median to our items. `position` was the child index direction we were headed in during recursion.
        self.items.insert(position, new_key_to_these_split_children);
        // add split's new Node to our children one place up from `position`
        self.children
            .insert(position + 1, Box::new(new_child_node_we_made));
        // increment children count
        self.num_children += 1;
    }
    // (1) a split did not echo up to us
    return None;
}
