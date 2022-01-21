mod lru;
mod linked;
mod stack;
mod queue;
mod recursion;
mod sort;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::ops::Index;
    use crate::lru::{Cache, List};
    use crate::linked::Linked;
    use crate::lru::Lru;
    use crate::queue::{ArrayQueue, Queue};
    use super::{lru, linked};
    use crate::stack;
    use crate::stack::Stack;
    use crate::recursion;
    use crate::sort;

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

    // 栈 表达式求值/括号匹配/前进后退
    #[test]
    fn stack() {
        // 创建最大容量为 5 的栈
        let mut s: Stack<i32> = stack::Stack::new(5);
        // 入栈
        s.push(1);
        s.push(2);
        s.push(3);
        s.push(4);
        s.push(5);
        assert_eq!(5, s.len);
        assert_eq!(5, s.count);
        // 出栈
        assert_eq!(Some(5), s.pop());
        assert_eq!(Some(4), s.pop());
        assert_eq!(Some(3), s.pop());
        assert_eq!(Some(2), s.pop());
        assert_eq!(Some(1), s.pop());
        assert_eq!(5, s.len);
        assert_eq!(0, s.count);
        // 模拟浏览器 前进后退
        let mut b = stack::Browser::new();
        // 浏览网页
        b.browse(1);
        b.browse(2);
        b.browse(3);
        b.browse(4);
        b.browse(5);
        // 后退前进
        assert_eq!(Some(4), b.backward());
        assert_eq!(Some(5), b.forward());
        assert_eq!(Some(4), b.backward());
        assert_eq!(Some(3), b.backward());
        assert_eq!(Some(2), b.backward());
        assert_eq!(Some(3), b.forward());
        assert_eq!(Some(4), b.forward());
        // 括号匹配
        let s = "sdf{123[xcv]zxv(fds)[11{22}3]4}55";
        assert_eq!(true, stack::brackets_match(s));
        let s = "[{()}([])]";
        assert_eq!(true, stack::brackets_match(s));
        let s = "{sdf[xv}(vxc)zxc]2131";
        assert_eq!(false, stack::brackets_match(s));
        let s = "[({)]";
        assert_eq!(false, stack::brackets_match(s));
        // 计算表达式
        let s = "3 + 5 * 8 - 6";
        assert_eq!(Some(3 + 5 * 8 - 6), stack::calculate(s));
        let s = "9 / 5 + 3 - 1 + 3 * 2 - 1";
        assert_eq!(Some(9 / 5 + 3 - 1 + 3 * 2 - 1), stack::calculate(s))
    }

    // 队列
    #[test]
    fn queue() {
        // 动态数组实现
        let mut v: Queue<List<i32>, i32> = Queue::new(3);
        v.enqueue(1);
        v.enqueue(2);
        v.enqueue(3);
        assert_eq!(false, v.enqueue(4));
        assert_eq!(v.dequeue(), Some(1));
        assert_eq!(v.dequeue(), Some(2));
        assert_eq!(v.dequeue(), Some(3));
        assert_eq!(v.dequeue(), None);
        // 链表数组实现
        let mut v: Queue<Linked<i32>, i32> = Queue::new(3);
        v.enqueue(1);
        v.enqueue(2);
        v.enqueue(3);
        assert_eq!(false, v.enqueue(4));
        assert_eq!(v.dequeue(), Some(1));
        assert_eq!(v.dequeue(), Some(2));
        assert_eq!(v.dequeue(), Some(3));
        assert_eq!(v.dequeue(), None);
        // 数组实现
        let mut v: ArrayQueue<i32> = ArrayQueue::new();
        v.enqueue(1);
        v.enqueue(2);
        v.enqueue(3);
        v.enqueue(4);
        v.enqueue(5);
        v.enqueue(6);
        v.enqueue(7);
        v.enqueue(8);
        v.enqueue(9);
        assert_eq!(false, v.enqueue(10));
        assert_eq!(v.dequeue(), Some(1));
        assert_eq!(v.dequeue(), Some(2));
        assert_eq!(v.dequeue(), Some(3));
        assert_eq!(v.dequeue(), Some(4));
        assert_eq!(v.dequeue(), Some(5));
        assert_eq!(v.dequeue(), Some(6));
        assert_eq!(v.dequeue(), Some(7));
        assert_eq!(v.dequeue(), Some(8));
        assert_eq!(v.dequeue(), Some(9));
        assert_eq!(v.dequeue(), None);
    }

    // 递归
    #[test]
    fn recursion() {
        // 座位问题 f(n) = f(n-1) + 1 f(1) = 1 体现逻辑 实际 fn(n) = n
        assert_eq!(5, recursion::f(5));
        assert_eq!(5, recursion::find_seat(5));
        // 爬楼梯问题
        assert_eq!(recursion::climbing_stairs_v1(13), 377);
        let mut h: HashMap<u32, u32> = HashMap::new();
        assert_eq!(recursion::climbing_stairs_v2(13, &mut h), 377);
        assert_eq!(recursion::climbing_stairs_v3(13), 377);
    }

    // 排序
    #[test]
    fn sort() {
        // 冒泡
        let mut v = vec![2, 4, 6, 2, 1, 4, 7, 2, 9, 1];
        sort::bubble(&mut v);
        assert_eq!(vec![1, 1, 2, 2, 2, 4, 4, 6, 7, 9], v);
        // 插入
        let mut v = vec![4, 5, 6, 1, 2, 3];
        sort::bubble(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], v);
        // 选择
        let mut v = vec![4, 5, 6, 1, 2, 3];
        sort::selection(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5, 6], v);
    }
}
