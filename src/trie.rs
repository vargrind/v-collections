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
    children: Vec<Rc<RefCell<TrieNode<V>>>>,
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
        let mut node: Option<&TrieNode<V>> = Some(&self.root);
        let mut rest = &key[..];
        while node.is_some() {
            let current = node.unwrap();
            if rest.len() == 0 {
                return current.value.as_ref();
            }
            rest = &rest[0..current.prefix.len()];
            for other in current.children.iter().map(|n| n.borrow()) {
                
            }
        }
        None
    }
    
    /// gets the value of a key as mutable
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        None
    }

    /// checks if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// sets a key to a value
    /// returns the key evicted if there was already a key.
    pub fn set(&mut self, key: &str, val: V) -> Option<V> {
        None
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
            size += other.borrow().size();
        }
        return size;
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
