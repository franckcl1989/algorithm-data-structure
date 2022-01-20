use std::marker::PhantomData;

// 队列存储结构
pub trait QueueCache<T> {
    fn new() -> Self where Self: Sized;
    fn enqueue(&mut self, n: T);
    fn dequeue(&mut self) -> Option<T>;
}

// 队列结构 方便直接用 linked 约束 Clone 等
pub struct Queue<R, T> where R: QueueCache<T>, T: Clone + PartialOrd + PartialEq {
    size: usize,
    len: usize,
    cache: R,
    mark: PhantomData<T>,
}

// 实现队列
impl<R, T> Queue<R, T> where R: QueueCache<T>, T: Clone + PartialOrd + PartialEq {
    pub fn new(n: usize) -> Self {
        Queue {
            len: 0,
            size: n,
            cache: QueueCache::new(),
            mark: Default::default(),
        }
    }
    // 入队
    pub fn enqueue(&mut self, n: T) -> bool {
        if self.len == self.size {
            false
        } else {
            self.cache.enqueue(n);
            self.len += 1;
            true
        }
    }
    // 出队
    pub fn dequeue(&mut self) -> Option<T> {
        self.cache.dequeue()
    }
}

// 数组实现队列 长度也是数组的类型参数 这里配置一个固定 10
#[derive(Debug)]
pub struct ArrayQueue<T> where T: Default + Copy {
    tail: usize,
    head: usize,
    cache: [T; 10],
    size: usize,
}

impl<T> ArrayQueue<T> where T: Default + Copy {
    // 创建 默认 size 10
    pub fn new() -> Self {
        ArrayQueue {
            tail: 0,
            head: 0,
            cache: [T::default(); 10],
            size: 10,
        }
    }
    // 入队
    pub fn enqueue(&mut self, n: T) -> bool {
        // 循环队列会浪费一个存储空间
        // 队列满 条件为 (tail + 1) % n == head
        if (self.tail + 1) % self.size == self.head {
            false
        } else {
            self.cache[self.tail] = n;
            // tail = (tail + 1) % n
            self.tail = (self.tail + 1) % self.size;
            true
        }
    }
    // 出队
    pub fn dequeue(&mut self) -> Option<T> {
        if self.head == self.tail {
            None
        } else {
            let n = self.cache[self.head];
            self.head = (self.head + 1) % self.size;
            Some(n)
        }
    }
}