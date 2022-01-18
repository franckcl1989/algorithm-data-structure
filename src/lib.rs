mod lru;

#[cfg(test)]
mod tests {
    use super::lru;

    #[test]
    // 基于动态数组实现LRU
    fn lru() {
        let mut lru = lru::Lru::new(5);
        lru.lookup(0);
        assert_eq!(&vec![0], lru.cache());
        lru.lookup(1);
        assert_eq!(&vec![1, 0], lru.cache());
        lru.lookup(2);
        assert_eq!(&vec![2, 1, 0], lru.cache());
        lru.lookup(3);
        assert_eq!(&vec![3, 2, 1, 0], lru.cache());
        lru.lookup(4);
        assert_eq!(&vec![4, 3, 2, 1, 0], lru.cache());
        lru.lookup(2);
        assert_eq!(&vec![2, 4, 3, 1, 0], lru.cache());
    }
}
