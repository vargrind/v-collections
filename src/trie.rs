use std::{rc::Rc, cell::RefCell};

#[derive(Debug, Clone)]
pub struct KeyExistsError;

#[derive(Debug, Clone)]
pub struct KeyNotFoundError;

/// compressed prefix tree
///
/// holds arbitrary values, uses string keys
/// common slices of stored keys are compressed by
/// not storing duplicates of those common slices.
pub struct Trie<V> {
    /// tree root
    /// this will always be a node with the empty string.
    root: TrieNode<V>,
}

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
    fn get(&self, key: &str) -> Option<&V> {
        self.root.get(key)
    }
    
    /// gets the value of a key as mutable
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        self.root.get_mut(key)
    }

    /// checks if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// sets a key to a value
    /// returns the key evicted if there was already a key.
    pub fn set(&mut self, key: &str, val: V) -> Option<V> {
        self.root.insert(key, val)
    }

    /// removes a key
    ///
    /// Ok() if key existed, Err() otherwise
    pub fn remove(&mut self, key: &str) -> Result<(V), KeyNotFoundError> {
        Err(KeyNotFoundError)
    }

    pub fn size(&self) -> usize {
        return self.root.size();
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
        let rest = &key[0..self.prefix.len()];
        let leaf = self.leaf(rest);
        match leaf {
            None => None,
            Some(node) => {
                node.get(rest)
            }
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
        let rest = &key[0..self.prefix.len()];
        let leaf = self.leaf_mut(rest);
        match leaf {
            None => None,
            Some(node) => {
                node.get_mut(rest)
            }
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
        let rest = &key[0..self.prefix.len()];
        let leaf = self.leaf_mut(key);
        // still longer than leaf, and leaf exists
        if leaf.is_some() {
            return leaf.unwrap().insert(rest, value);
        }
        // shorter than a valid leaf split target
        let split = self.insert_split_target(rest);
        if split.is_some() {
            let (idx, node) = split.unwrap();
            let mut inject = TrieNode {
                prefix: (&rest[(rest.len() - 1)..(node.prefix.len() - rest.len())]).to_owned(),
                children: Vec::new(),
                value: Some(value),
            };
            let moved = std::mem::replace(&mut self.children[idx], inject);
            self.children[idx].children.push(moved);
            return None;
        }
        // neither a leaf is our prefix, nor are we a leaf prefix, inject new leaf.
        let mut inject = TrieNode {
            prefix: rest.to_owned(),
            children: Vec::new(),
            value: Some(value),
        };
        self.children.push(inject);
        return None;
    }

    fn insert_split_target(&mut self, key: &str) -> Option<(usize, &mut Self)> {
        self.children.iter_mut().enumerate().find(|(idx, node)| node.prefix.starts_with(key))
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn insertion_retrieval() {
        let mut trie: Trie<i32> = Trie::new();
        let v1 = vec!["a", "b", "c", "ab", "ac", "abc", "abcde"];
        let v2  = vec![1, 2, 3, 4, 5, 6, 7];
        for i in 0..7 {
            trie.set(v1[i], v2[i]);
        }
        for i in 0..7 {
            assert_eq!(trie.get(v1[i]), Some(&v2[i]))
        }
    }
}
