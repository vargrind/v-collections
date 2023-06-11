pub mod btree;
pub mod circular;
pub mod rbtree;
pub mod trie;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
