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
        Trie {
            root: None,
        }
    }
    
    /// gets the value of a key
    pub fn get(&self, key: &str) -> Option<V> {
        
    }

    /// sets a key to a value
    pub fn set(&mut self, key: &str, val: V) {
        
    }

    /// tries to set
    /// 
    /// returns an error if key is already set.
    pub fn try_set(&mut self, key: &str, val: V) -> Result<(), KeyExistsError> {
        
    }
    
    /// checks if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// removes a key
    /// 
    /// Ok() if key existed, Err() otherwise
    pub fn remove(&self, key: &str) -> Result<(), KeyNotFoundError> {

    }
    
}

impl<V> TrieNode<V> {
    
}
