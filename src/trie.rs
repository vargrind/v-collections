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
    root: Option<TrieNode<V>>,
    size: u32,
}

struct TrieNode<V> {
    prefix: String,
    children: Vec<TrieNode<V>>,
    value: Option<V>,
}

impl<V> Trie<V> {
    /// constructs an empty prefix tree
    pub fn new() -> Self {
        Trie { root: None, size: 0 }
    }

    /// gets the value of a key
    pub fn get(&self, key: &str) -> Option<&V> {
        if self.root.is_none() {
            return None;
        }
        let mut node = self.root.as_ref().unwrap();
        loop {
            if !key.starts_with(&node.prefix) {
                break;
            }
            if node.prefix.len() == key.len() {
                if node.value.is_none(){
                    return None;
                }
                else {
                    return Some(node.value.as_ref().unwrap());
                }
            }
            if node.children.is_empty() {
                break;
            }
            let rest = &key[node.prefix.len()..];
            let index = node.children.binary_search_by(|n| n.prefix[..].cmp(rest));
            match index {
                Err(_) => break,
                Ok(i) => node = &node.children[i],
            }
        }
        None
    }

    /// gets the value of a key as mutable
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        if self.root.is_none() {
            return None;
        }
        let mut node = self.root.as_mut().unwrap();
        loop {
            if !key.starts_with(&node.prefix) {
                break;
            }
            if node.prefix.len() == key.len() {
                if node.value.is_none(){
                    return None;
                }
                else {
                    return Some(node.value.as_mut().unwrap());
                }
            }
            if node.children.is_empty() {
                break;
            }
            let rest = &key[node.prefix.len()..];
            let index = node.children.binary_search_by(|n| n.prefix[..].cmp(rest));
            match index {
                Err(_) => break,
                Ok(i) => node = &mut node.children[i],
            }
        }
        None
    }

    /// checks if a key exists
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    /// sets a key to a value
    /// returns the key evicted if there was already a key.
    pub fn set(&mut self, key: &str, val: V) -> Option<V> {
        
    }

    /// removes a key
    ///
    /// Ok() if key existed, Err() otherwise
    pub fn remove(&mut self, key: &str) -> Result<(V), KeyNotFoundError> {
        
    }
}

impl<V> TrieNode<V> {
    fn insert(&mut self, node: Self) {
        let pos = {
            let mut i = 0;
            loop {
                if i >= self.children.len() {
                    break i;
                }
                if self.children[i].prefix >= node.prefix {
                    break i;
                }
                i += 1;
            }
        };
        self.children.insert(pos, node); 
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
