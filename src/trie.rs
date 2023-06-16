use std::cell::RefCell;

#[derive(Debug, Clone)]
struct KeyExistsError {}

#[derive(Debug, Clone)]
struct KeyNotFoundError {}

/// compressed prefix tree
///
/// holds arbitrary values, uses string keys
/// common slices of stored keys are compressed by
/// not storing duplicates of those common slices.
pub struct Trie<V> {
    root: Option<TrieNode<V>>,
}

struct TrieNode<V> {
    prefix: String,
    children: Vec<TrieNode<V>>,
    value: V,
}

impl<V> Trie<V> {
    /// constructs an empty prefix tree
    pub fn new() -> Self {
        Trie { root: None }
    }

    /// gets the value of a key
    pub fn get(&self, key: &str) -> Option<&V> {
        match(self.root) {
            None => None,
            Some(r) => r.lookup(key),
        }
    }

    /// gets the value of a key as mutable
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match(self.root) {
            None => None,
            Some(mut r) => r.lookup_mut(key),
        }
    }

    /// sets a key to a value
    /// returns the key evicted if there was already a key.
    pub fn set(&mut self, key: &str, val: V) -> Option<V> {
        
    }

    /// checks if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// removes a key
    ///
    /// Ok() if key existed, Err() otherwise
    pub fn remove(&mut self, key: &str) -> Result<(V), KeyNotFoundError> {
        match(self.root) {
            None => Err(KeyNotFoundError {  }),
            Some(mut v) => match(v.evict(key)) {
                None => Err(KeyNotFoundError {}),
                Some(thing) => Ok(thing),
            },
        }
    }
}

impl<V> TrieNode<V> {
    fn evict(&mut self, key: &str) -> Option<V> {

    }

    fn insert(&mut self, key: &str, val: V) -> Option<V> {

    }
    
    fn lookup(&self, key: &str) -> Option<&V> {
        match(self.node(key)) {
            None => None,
            Some(n) => Some(&n.value),
        }
    }

    fn lookup_mut(&mut self, key: &str) -> Option<&mut V> {
        match(self.node(key)) {
            None => None,
            Some(n) => Some(&mut n.value),
        }
    }
    
    fn node(&self, key: &str) -> Option<&Self> {
        
    }

}
