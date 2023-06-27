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
        match self.key_node(key) {
            Some(n) => Some(&n.value),
            None => None,
        }
    }

    /// gets the value of a key as mutable
    pub fn get_mut(&mut self, key: &str) -> Option<&mut V> {
        match self.key_node(key) {
            Some(mut n) => Some(&mut n.value),
            None => None,
        }
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

    fn key_node(&self, key: &str) -> Option<TrieNode<V>> {
        let mut current_node = self.root;
        while let Some(node) = current_node {
            if !key.starts_with(&node.prefix) {
                break;
            }
            if node.prefix.len() == key.len() {
                return Some(node);
            }
            if node.children.is_empty() {
                break;
            }
            let rest = &key[node.prefix.len()..];
            let index = node.children.binary_search_by(|n| n.prefix[..].cmp(rest));
            match index {
                Err(_) => break,
                Ok(i) => current_node = Some(node.children[i]),
            }
        }
        None
    }
}

impl<V> TrieNode<V> {
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
