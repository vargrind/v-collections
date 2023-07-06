use std::ops::AddAssign;

#[derive(Debug, Clone)]
pub struct KeyExistsError;

#[derive(Debug, Clone)]
pub struct KeyNotFoundError;

/// compressed prefix tree
///
/// holds arbitrary values, uses string keys
/// common slices of stored keys are compressed by
/// not storing duplicates of those common slices.
#[derive(Debug)]
pub struct Trie<V> {
    /// tree root
    /// this will always be a node with the empty string.
    root: TrieNode<V>,
}

#[derive(Debug)]
struct TrieNode<V> {
    prefix: String,
    children: Vec<TrieNode<V>>,
    value: Option<V>,
}

impl<V> Trie<V> {
    /// constructs an empty prefix tree
    pub fn new() -> Self {
        Trie {
            root: TrieNode {
                value: None,
                prefix: "".to_owned(),
                children: Vec::new(),
            },
        }
    }

    /// gets the value of a key
    #[inline]
    fn get(&self, key: &str) -> Option<&V> {
        self.root.get(key)
    }

    /// gets the value of a key as mutable
    #[inline]
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        self.root.get_mut(key)
    }

    /// checks if a key exists
    #[inline]
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// sets a key to a value
    /// returns the key evicted if there was already a key.
    #[inline]
    pub fn set(&mut self, key: &str, val: V) -> Option<V> {
        self.root.insert(key, val)
    }

    /// removes a key
    ///
    /// Ok() if key existed, Err() otherwise
    #[inline]
    pub fn remove(&mut self, key: &str) -> Result<V, KeyNotFoundError> {
        match self.root.remove(key) {
            None => Err(KeyNotFoundError),
            Some(data) => Ok(data),
        }
    }

    /// Gets the size of the tree in terms of nodes.
    #[inline]
    pub fn size(&self) -> usize {
        self.root.size()
    }
}

impl<V> TrieNode<V> {
    fn size(&self) -> usize {
        let mut size = 1;
        for other in self.children.iter() {
            size += other.size();
        }
        return size;
    }

    fn get(&self, key: &str) -> Option<&V> {
        if key == self.prefix {
            return self.value.as_ref();
        }
        let rest = &key[self.prefix.len()..];
        let leaf = self.leaf(rest);
        match leaf {
            None => None,
            Some(node) => node.get(rest),
        }
    }

    fn leaf(&self, key: &str) -> Option<&Self> {
        for node in self.children.iter() {
            if key.starts_with(&node.prefix) {
                return Some(&node);
            }
        }
        None
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        if key == self.prefix {
            return self.value.as_mut();
        }
        let rest = &key[self.prefix.len()..];
        let leaf = self.leaf_mut(rest);
        match leaf {
            None => None,
            Some(node) => node.get_mut(rest),
        }
    }

    fn leaf_mut(&mut self, key: &str) -> Option<&mut Self> {
        for node in self.children.iter_mut() {
            if key.starts_with(&node.prefix) {
                return Some(node);
            }
        }
        None
    }

    fn insert(&mut self, key: &str, value: V) -> Option<V> {
        if key == self.prefix {
            return self.value.replace(value);
        }
        let rest = &key[self.prefix.len()..];
        let leaf = self.leaf_mut(rest);
        // still longer than leaf, and leaf exists
        if leaf.is_some() {
            return leaf.unwrap().insert(rest, value);
        }
        // shorter than a valid leaf split target
        let split = self.insert_split_target(rest);
        if split.is_some() {
            let (idx, node) = split.unwrap();
            let inject = TrieNode {
                prefix: (&rest[(rest.len() - 1)..(node.prefix.len() - rest.len())]).to_owned(),
                children: Vec::new(),
                value: Some(value),
            };
            let moved = std::mem::replace(&mut self.children[idx], inject);
            self.children[idx].children.push(moved);
            return None;
        }
        // neither a leaf is our prefix, nor are we a leaf prefix, inject new leaf.
        let inject = TrieNode {
            prefix: rest.to_owned(),
            children: Vec::new(),
            value: Some(value),
        };
        self.children.push(inject);
        return None;
    }

    fn insert_split_target(&mut self, key: &str) -> Option<(usize, &mut Self)> {
        self.children
            .iter_mut()
            .enumerate()
            .find(|(_idx, node)| node.prefix.starts_with(key))
    }
    
    fn remove(&mut self, key: &str) -> Option<V> {
        if key == self.prefix {
            // us, this should only happen on first node. eject value.
            return self.value.take();
        }
        self.remove_internal(&key[self.prefix.len()..])
    }

    fn remove_internal(&mut self, key: &str) -> Option<V> {
        // get leaf node
        let rest = &key[self.prefix.len()..];
        let leaf = self.leaf_mut(rest);
        if leaf.is_none() {
            // not found, bail
            return None;
        }
        // unwrap it - this relies on local variable shadowing
        let leaf = leaf.unwrap();
        // leaf is not exact
        if leaf.prefix != rest {
            // kick it down
            return leaf.remove_internal(rest);
        }
        // leaf is exact
        // evict value
        let evicted = leaf.value.take();
        // some options
        match leaf.children.len() {
            0 => {
                // empty, evict
                let prefix = leaf.prefix.to_owned();
                self.evict_node_with_prefix(&prefix);
            }
            1 => {
                // 1 node. it should take the node below it.
                leaf.take_below();
            }
            _ => {
                // more than 1 node; do nothing, as it needs to stay there to be a split/branching node.
            }
        }
        match self.children.len() {
            1 => {
                // we only have one child, we should take the node we just read
                self.take_below();
            }
            _ => {
                // can't do anything, we need to be a branching node
            }
        }
        // return evicted value
        evicted
    }

    fn evict_node_with_prefix(&mut self, prefix: &str) {
        self.children.swap_remove(
            self.children
                .iter()
                .enumerate()
                .find(|(_idx, n)| n.prefix == prefix)
                .unwrap()
                .0,
        );
    }

    fn take_below(&mut self) {
        // this only makes sense if we only have 1 node.
        assert!(self.children.len() == 1);
        // take the node from below
        let taken = std::mem::replace(&mut self.children[0].children, Vec::new());
        // remove the node we have
        let node = self.children.remove(0);
        // steal their prefix
        let prefix = node.prefix.to_owned();
        // drop that node just in case
        std::mem::drop(node);
        // replace our children with that node
        self.children = taken;
        // append their prefix to ours
        self.prefix.add_assign(&prefix);
    }
}

#[cfg(test)]
mod tests {

    use test::Bencher;

    use super::*;

    #[test]
    fn insertion_retrieval() {
        let mut trie = Trie::new();
        let v1 = vec!["a", "ab", "ac", "b", "c", "abc", "abcde", "abced"];
        let v2 = vec![1, 2, 3, 4, 5, 6, 7, 9];
        for i in 0..8 {
            trie.set(v1[i], v2[i]);
        }
        for i in 0..8 {
            assert_eq!(trie.get(v1[i]), Some(&v2[i]));
        }
        assert_eq!(trie.size(), 9);
        trie.set(v1[3], 33);
        assert_eq!(trie.get(v1[3]), Some(&33));
        assert_eq!(trie.size(), 9);
    }

    #[test]
    fn insertion_deletion() {
        let mut trie = Trie::new();
        let v1 = vec!["a", "ab", "ac", "b", "c", "abc", "abcde", "abced"];
        let v2 = vec![1, 2, 3, 4, 5, 6, 7, 9];
        for i in 0..8 {
            trie.set(v1[i], v2[i]);
        }
        for i in 0..8 {
            assert_eq!(trie.get(v1[i]), Some(&v2[i]));
        }
        assert_eq!(trie.size(), 9);
        let removed = trie.remove("abcd");
        assert!(removed.is_err());
        let removed = trie.remove("abcde");
        assert_eq!(removed.ok(), Some(7));
        assert_eq!(trie.size(), 7);
        let removed: Result<i32, KeyNotFoundError> = trie.remove("c");
        assert_eq!(removed.ok(), Some(5));
        assert_eq!(trie.size(), 6);
        let removed = trie.remove("abcde");
        assert!(removed.is_err());
        assert_eq!(trie.size(), 6);
    }

    #[bench]
    fn sequential_number_strings(bencher: &mut Bencher) {
        let mut v = vec![];
        for i in 0..10000 {
            let str = i.to_string();
            v.push(str);
        }
        bencher.iter(|| {
            let mut tree = Trie::new();
            v.iter().for_each(|s| { tree.set(s, 1); });
        });
    }

    #[bench]
    fn sequential_numerical_strings_dupe(bencher: &mut Bencher) {
        let mut v = vec![];
        for _ in 0..=1 {
            for i in 0..5000 {
                let str = i.to_string();
                v.push(str);
            }
        }
        bencher.iter(|| {
            let mut tree = Trie::new();
            v.iter().for_each(|s| { tree.set(s, 1); });
        });
    }

    #[bench]
    fn exact_string_dupe(bencher: &mut Bencher) {
        let mut v = vec![];
        for _ in 0..10000 {
            let str = 1234567890.to_string();
            v.push(str);
        }
        bencher.iter(|| {
            let mut tree = Trie::new();
            v.iter().for_each(|s| { tree.set(s, 1); });
        });
    }

    #[bench]
    fn dupe_strings_under_load(bencher: &mut Bencher) {
        let mut v = vec![];
        for i in 0..10000 {
            let str = i.to_string();
            v.push(str);
        }
        let mut tree = Trie::new();
        v.iter().for_each(|s| { tree.set(s, 1); });
        let mut v = vec![];
        for _ in 0..10000 {
            let str = 9999.to_string();
            v.push(str);
        }
        bencher.iter(|| {
            v.iter().for_each(|s| { tree.set(s, 1); });
        });
    }
    
    #[bench]
    fn dupe_longer_strings_under_load(bencher: &mut Bencher) {
        let mut v = vec![];
        for i in 0..10000 {
            let str = i.to_string();
            v.push(str);
        }
        let mut tree = Trie::new();
        v.iter().for_each(|s| { tree.set(s, 1); });
        let mut v = vec![];
        for _ in 0..10000 {
            let str = 9999999999999u64.to_string();
            v.push(str);
        }
        bencher.iter(|| {
            v.iter().for_each(|s| { tree.set(s, 1); });
        });
    }
}
