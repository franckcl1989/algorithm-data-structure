use crate::sort::count;

// 堆 heap （数据结构与操作系统、内存上的堆概念不同）
// 堆是一种特殊的树，应用范围非常广泛最经典的是堆排序，堆排序是一种原地的时间复杂度为 O(nlogn) 的算法
// 堆的要求：堆是完全二叉树，每个节点的值必须大于等于或者小于等于它子树中每个节点的值
// 1. 堆是完全二叉树：除最后一层，其他层的节点个数都是满的且最后一层节点靠左排列
// 2. 堆的每个节点的值都大于等于或者小于等于它左右子节点的值（大于的叫大顶堆，小于的叫小顶堆）
// 堆的结构：完全二叉树适合使用数组来存储，使用数组存储非常节省内存空间，而堆就是完全二叉树
// 下标为 i 的节点其左子节点下标为 i * 2，右子节点下标为 i * 2 + 1
// 堆支持的操作：插入元素
// 当插入元素放到堆的最后可能不符合堆的特性，所以要对堆进行调整使其重新满足堆的特性，这个过程就叫做堆化
// 堆化有两种方式：自上而下、自下而上，堆化顺着节点路径不断比较和替换，新插入的节点与父节点进行比较如何不符合（大顶堆大 于等于子节点、小顶堆小于等于）则进行交换位置
// 堆结构声明
#[derive(Debug)]
pub struct Heap<T> {
    // 容器（用 vec 简单）
    inner: Vec<T>,
    // 最大容量（初始化用）
    max: usize,
    // 当前用量（当前容量）
    count: usize,
}

impl<T> Heap<T> where T: Ord + Clone {
    // 创建堆 参数：最大容量
    pub fn new(size: usize) -> Self {
        Heap {
            inner: Vec::with_capacity(size + 1),
            max: size,
            count: 0,
        }
    }
    // 插入堆化（与父节点进行比较，根据结果交换位置）
    pub fn insert(&mut self, n: T) -> Option<()> {
        if self.count < self.max {
            self.inner.push(n.clone());
            // 下标从 1 开始
            self.count += 1;
            self.inner.insert(self.count, n);
            let mut i = self.count;
            // 如果 i /2 > 则长度满足，如果 i > i / 2 (插入节点大于父节点) 交换
            while i / 2 > 0 && self.inner[i] > self.inner[i / 2] {
                self.inner.swap(i, i / 2);
                i = i / 2;
            }
            // 完成交换返回 Some 代表成功
            return Some(());
        }
        // 返回 None 代表插入失败
        None
    }
    // 删除堆顶元素：把最后一个节点放到堆顶，然后利用父子节点比对
    pub fn remove(&mut self) -> Option<()> {
        if self.count == 0 {
            return None;
        }
        self.inner[1] = self.inner.pop().unwrap();
        let mut count = self.count - 1;
        let mut i = 1;
        loop {
            let mut max = 1;
            if i * 2 <= count && self.inner[i] < self.inner[i * 2] {
                max = i * 2;
            }
            if i * 2 + 1 <= count && self.inner[max] < self.inner[i * 2 + 1] {
                max = i * 2 + 1;
            }
            if max == i {
                break;
            }
            self.inner.swap(i, max);
        }
        Some(())
    }
}

// 堆排序 原地排序 时间复杂度 O(nlogn) 不稳定
// 相较快速排序堆排序对 CPU 缓存不友好，同样数据规模堆排序的元素交换次数比快排多
// 堆排序分为两个步骤：建堆和排序
// 建堆
// 1. 从下往上：初始堆只包含下标为 1 的数据，然后依次插入下标 2 - n 的数据
// 2. 从上往下：从最后一个节点依次堆化（子节点与当前节点进行比对，交换数据）
// 排序
// 建堆结束之后数组中的数据已经是按照大顶堆的特性来组织，数组中第一个元素就是堆顶也就是最大的元素
// 把下标为 n 的元素放到堆顶，然后堆化剩余的 n - 1个元素，再取堆顶元素放到下标为 n - 1的位置，不断重复直到只剩下标为1的元素排序完成