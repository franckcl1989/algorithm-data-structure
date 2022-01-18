// 基于动态数组实现LRU
// 定义容量和当前长度（没有采用动态获取）
#[derive(Debug)]
pub struct Lru<T> where T: PartialEq + Ord {
    cache: Vec<T>,
    size: usize,
    len: usize,
}

impl<T> Lru<T> where T: PartialEq + Ord {
    // 根据容量初始化
    pub fn new(n: usize) -> Self {
        Lru {
            cache: Vec::new(),
            size: n,
            len: 0,
        }
    }
    pub fn cache(&self) -> &Vec<T> {
        &self.cache
    }
    // 策略
    pub fn lookup(&mut self, x: T) {
        // 元素存在则将元素移动到 head
        if self.cache.contains(&x) {
            let mut index = 0;
            for (i, j) in self.cache.iter().enumerate() {
                if j == &x {
                    index = i;
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