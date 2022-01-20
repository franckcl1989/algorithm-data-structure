use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Index;
use crate::queue::QueueCache;

// 基于动态数组实现LRU
// 定义缓存 Trait
pub trait Cache<T> {
    fn new() -> Self where Self: Sized;
    fn contains(&mut self, n: &T) -> bool;
    fn remove(&mut self, i: usize) -> Option<T>;
    fn insert(&mut self, i: usize, n: T);
    fn pop(&mut self) -> Option<T>;
    fn index(&mut self, i: usize) -> Option<&T>;
}

// 包装 Vec
#[derive(Clone, Debug)]
pub struct List<T> where T: Clone + PartialOrd + PartialEq {
    inner: Vec<T>,
}

// 实现 Queue 特性
impl<T> QueueCache<T> for List<T> where T: Clone + PartialOrd + PartialEq {
    fn new() -> Self {
        List {
            inner: Vec::new()
        }
    }

    fn enqueue(&mut self, n: T) {
        self.inner.push(n);
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.inner.len() == 0 {
            None
        } else {
            self.remove(0)
        }
    }
}

// Vec => List 转换
impl<T> From<Vec<T>> for List<T> where T: Clone + PartialOrd + PartialEq {
    fn from(v: Vec<T>) -> Self {
        List {
            inner: v
        }
    }
}

// List => Vec 转换
impl<T> Into<Vec<T>> for List<T> where T: Clone + PartialOrd + PartialEq {
    fn into(mut self) -> Vec<T> {
        self.inner
    }
}

// 实现 Cache Trait
impl<T> Cache<T> for List<T> where T: Clone + PartialOrd + PartialEq {
    fn new() -> Self {
        List {
            inner: Vec::new()
        }
    }
    fn contains(&mut self, n: &T) -> bool {
        self.inner.contains(&n)
    }
    fn remove(&mut self, i: usize) -> Option<T> {
        Some(self.inner.remove(i))
    }
    fn insert(&mut self, i: usize, n: T) {
        self.inner.insert(i, n)
    }

    fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn index(&mut self, i: usize) -> Option<&T> {
        Some(self.inner.index(i))
    }
}

// 定义容量和当前长度（没有采用动态获取）
#[derive(Clone, Debug)]
pub struct Lru<C, T> where C: Cache<T>, T: Clone + PartialOrd + PartialEq {
    // cache 作为泛型 支持不同得数据结构
    cache: C,
    size: usize,
    len: usize,
    mark: PhantomData<T>,
}

impl<C, T> Lru<C, T> where C: Cache<T> + Debug, T: Clone + Debug + PartialOrd + PartialEq {
    // 根据容量初始化
    pub fn new(n: usize) -> Self {
        Lru {
            cache: Cache::new(),
            size: n,
            len: 0,
            mark: Default::default(),
        }
    }
    pub fn cache(&self) -> &C {
        &self.cache
    }
    // 策略
    pub fn lookup(&mut self, x: T) {
        // 元素存在则将元素移动到 head
        if self.cache.contains(&x) {
            let mut index = 0;
            for i in 0..self.len {
                if let Some(c) = self.cache.index(i) {
                    if c.clone() == x.clone() {
                        index = i;
                    }
                }
            }
            self.cache.remove(index);
            self.cache.insert(0, x);
        } else {
            // 容量够 直接添加
            if self.len < self.size {
                self.len += 1;
            } else {
                // 容量不足 先删除 tail 元素
                self.cache.pop();
            }
            self.cache.insert(0, x);
        }
    }
}