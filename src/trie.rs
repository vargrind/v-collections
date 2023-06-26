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
/// known limitations:
/// * value is an Option even though it isn't semantically necessary, this is for internal usage reasons.
pub struct Trie<V> {
    root: Option<TrieNode<V>>,
}

struct TrieNode<V> {
    prefix: String,
    children: Option<Vec<TrieNode<V>>>,
    value: Option<V>,
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
        match(self.root) {
            None => {
                let mut new_root = TrieNode {
                    prefix: "".to_string(),
                    children: None,
                    value: None,
                };
                new_root.insert(key, val);
                self.root = Some(new_root);
                None
            },
            Some(mut r) => r.insert(key, val),
        }
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
            Some(n) => Some(&n.value.unwrap()),
        }
    }

    fn lookup_mut(&mut self, key: &str) -> Option<&mut V> {
        match(self.node(key)) {
            None => None,
            Some(n) => Some(&mut n.value.unwrap()),
        }
    }
    
    fn node(&self, key: &str) -> Option<&Self> {
        if key.starts_with(self.prefix.as_str()) {
            if key.len() == self.prefix.len() {
                return Some(self);
            }
            if self.prefix.len() == 0 {
                return None;
            }
            let rest = &key[self.prefix.len()..];
            match self.children {
                None => None,
                Some(v) => {
                    for n in v {
                        if n.prefix.bytes().next().unwrap() == self.prefix.bytes().next().unwrap() {
                            return n.node(key);
                        }
                    }
                    None
                }
            }
        }
        else{
            None
        }
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
