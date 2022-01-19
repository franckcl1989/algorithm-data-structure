mod lru;
mod linked;

#[cfg(test)]
mod tests {
    use std::ops::Index;
    use crate::lru::Cache;
    use crate::linked::Linked;
    use crate::lru::Lru;
    use super::{lru, linked};

    #[test]
    // 基于动态数组/单链表实现LRU
    fn lru() {
        // 动态数组
        let mut lru: Lru<lru::List<_>, i32> = lru::Lru::new(5);
        lru.lookup(0);
        let v1 = vec![0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(1);
        let v1 = vec![1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(2);
        let v1 = vec![2, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(3);
        let v1 = vec![3, 2, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(4);
        let v1 = vec![4, 3, 2, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(2);
        let v1 = vec![2, 4, 3, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        // 单链表
        let mut lru: Lru<Linked<_>, i32> = lru::Lru::new(5);
        lru.lookup(0);
        let v1 = vec![0];
        let v2: Vec<i32> = lru.cache().clone().into();
        println!("{:?}", v2);
        assert_eq!(v1, v2);
        lru.lookup(1);
        let v1 = vec![1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(2);
        let v1 = vec![2, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(3);
        let v1 = vec![3, 2, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(4);
        let v1 = vec![4, 3, 2, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
        lru.lookup(2);
        let v1 = vec![2, 4, 3, 1, 0];
        let v2: Vec<i32> = lru.cache().clone().into();
        assert_eq!(v1, v2);
    }

    #[test]
    // 单链表 反转 有序合并 删除倒数第 N 节点 获取中间节点
    fn linked() {
        // 创建压入
        let mut v: Linked<i32> = linked::Linked::new();
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(3);
        // 索引查找
        assert_eq!(Some(&0), v.index(0));
        assert_eq!(Some(&1), v.index(1));
        assert_eq!(Some(&2), v.index(2));
        assert_eq!(Some(&3), v.index(3));
        assert_eq!(None, v.index(4));
        // 删除最后
        assert_eq!(Some(3), v.pop());
        assert_eq!(Some(2), v.pop());
        assert_eq!(Some(1), v.pop());
        assert_eq!(Some(0), v.pop());
        v.push(0);
        v.push(1);
        v.push(2);
        v.push(3);
        // 索引删除
        assert_eq!(Some(0), v.remove(0));
        assert_eq!(Some(1), v.remove(0));
        assert_eq!(Some(2), v.remove(0));
        assert_eq!(Some(3), v.remove(0));
        // Vec <=> Linked 转换
        let mut v = Linked::from(vec![0, 1, 2, 3, 4, 5]);
        let mut v: Vec<_> = v.into();
        println!("{:?}", v);
        // 删除倒数第 N 节点，这里通过长度查询（干查用双指针，快指针先走 n 快慢一起走 快到头 慢为 n - 1）
        let mut v = Linked::from(vec![0, 1, 2, 3, 4, 5]);
        let len = v.len();
        assert_eq!(Some(4), v.remove(len - 2));
        // 获取中间节点，这里通过长度查询(干查用双指针，起点 head 快走 2 步 慢走 1 步，快到头 慢为中间节点)
        let mut v = Linked::from(vec![1, 2, 3, 4, 5]);
        assert_eq!(Some(&3), v.index(v.len() / 2));
        // 合并两个有序列表
        let v1 = Linked::from(vec![0, 1, 1, 3, 3, 5, 7, 9]);
        let v2 = Linked::from(vec![1, 2, 2, 4, 4, 6, 8, 10]);
        let l = Linked::merge_order(v1, v2).unwrap();
        let v3: Vec<_> = l.into();
        assert_eq!(vec![0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9, 10], v3);
        // 索引地址插入
        let mut v = Linked::from(vec![0]);
        v.insert(0, 1);
        v.insert(1, 3);
        // 反转
        let mut v = Linked::from(vec![1, 2, 3, 4, 5]);
        v.reverse();
        let v1: Vec<_> = v.into();
        let v2 = vec![5, 4, 3, 2, 1];
        assert_eq!(v1, v2);
        let mut v = Linked::from(vec![1, 2, 3, 4, 5]);
        println!("{}", v.contains(&0));
        println!("{}", v.contains(&1));
        assert_eq!(true, v.contains(&1));
        assert_eq!(false, v.contains(&0));
    }
}
