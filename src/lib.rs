mod lru;
mod linked;

#[cfg(test)]
mod tests {
    use crate::linked::Linked;
    use super::{lru, linked};

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

    #[test]
    // 单链表 反转 有序合并 删除倒数第 N 节点 获取中间节点
    fn linked() {
        let mut v: Linked<i32> = linked::Linked::new();
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(3);
        // println!("{:?}", v);
        // v.pop();
        // println!("{:?}", v);
        // v.pop();
        // println!("{:?}", v);
        // v.pop();
        // println!("{:?}", v);
        println!("{:?}", v.index(1));
    }
}
