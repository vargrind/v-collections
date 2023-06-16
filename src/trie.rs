#[derive(Debug, Clone)]
struct KeyExistsError {}

#[derive(Debug, Clone)]
struct KeyNotFoundError {}

/// compressed prefix tree
///
/// holds arbitrary values, uses string keys
/// common slices of stored keys are compressed by
/// not storing duplicates of those common slices.
/// 
/// known issues:
/// 1. TrieNode uses Option<> for internal usages, even though it shouldn't actually exist if it's empty.
/// 2. Everything is using mut to pass mutable references out, so this tree allows mutation / modification of internal contents
pub struct Trie<V> {
    root: Option<TrieNode<V>>,
}

struct TrieNode<V> {
    prefix: String,
    children: Vec<TrieNode<V>>,
    value: Option<V>,
}

impl<V> Trie<V> {
    /// constructs an empty prefix tree
    pub fn new() -> Self {
        Trie { root: None }
    }

    /// gets the value of a key
    pub fn get(&self, key: &str) -> Option<&mut V> {
        match(self.root) {
            None => None,
            Some(r) => r.lookup(key),
        }
    }

    /// sets a key to a value
    pub fn set(&mut self, key: &str, val: V) {
        
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
    
    fn lookup(&self, key: &str) -> Option<&mut V> {
        match(self.node(key)) {
            None => None,
            Some(n) => n.value.as_mut(),
        }
    }

    fn node(&self, key: &str) -> Option<Self> {
        
    }
}
