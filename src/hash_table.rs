use std::marker::PhantomData;
use std::collections::hash_map::DefaultHasher;
use std::collections::LinkedList;
use std::hash::{Hash, Hasher};

// 散列表
// key => 散列函数 => 数组下标 => 数组
// 时间复杂度：O(1)
// 解决散列冲突：开放寻址、链表法
// 开放寻址：线性探测、二次探测、双重散列
// 线性探测：步长为1寻找下一个空闲位置
// 二次探测：步长为二次方寻找下一个空闲位置
// 双重散列：使用多个散列函数寻找空闲位置
// 装载因子：装载因子 = 元素个数 / 长度 用以表示空闲位置多少，装载因子越大表示位置越少则冲突越多性能下降
// 链表法：key => 散列函数 => 数组下标 => 数组 => 链表，每个元素插入链表，查找删除遍历链表
// 散列表设计：
// 1. 散列函数设计不能太复杂同时生成得散列值尽可能随机均匀分布
// 2. 避免时间复杂度降低要平衡好装载因子得大小进行动态得扩容
// 3. 避免低效扩容要有适当得动态扩容方法，例如扩容时申请空间后不直接进行数据复制而是每次插入时候复制一个元素（查询先查新后查旧）
// 4. 解决散列冲突：开放寻址适合数据量小、装载因子小得情况，链表适合数据量大、存储对象大得情况
// 工业级应该具备得特性：
// 1. 支持快速查询、插入、删除等操作
// 2. 内存占用合理，不浪费内存
// 3. 性能稳定，不会退化到时间复杂度很高得情况
// 4. 一个合适得散列函数
// 5. 有装载因子阈值、有动态扩容策略
// 6. 适合得散列冲突解决办法
pub struct HashTable<K, V> where K: Hash + PartialOrd + PartialEq + Clone, V: Clone + PartialEq + PartialOrd {
    capacity: usize,
    store: Vec<LinkedList<(K, V)>>,
    marker: PhantomData<K>,
}

fn hash<K>(key: K) -> i32 where K: Hash {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() % 8) as i32
}

impl<K, V> HashTable<K, V> where K: Hash + PartialOrd + PartialEq + Clone, V: Clone + PartialEq + PartialOrd {
    pub fn new() -> Self {
        let mut v = Vec::with_capacity(8);
        for _ in 0..8 {
            v.push(LinkedList::new());
        }
        HashTable {
            capacity: 0,
            store: v,
            marker: Default::default(),
        }
    }
    pub fn insert(&mut self, k: K, v: V) {
        let index = hash(&k) as usize;
        if index > self.store.len() + 1 {
            self.store.insert(index, LinkedList::new());
        }
        if !self.store[index].contains(&(k.clone(), v.clone())) {
            self.store[index].push_back((k, v));
        }
    }
    pub fn get(&mut self, k: K) -> Option<&V> {
        let index = hash(&k) as usize;
        if index < self.store.len() {
            for n in self.store[index].iter_mut() {
                if k == n.0 {
                    return Some(&n.1);
                }
            }
        }
        None
    }
}