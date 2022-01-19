use std::fmt::Debug;

// 定义节点
#[derive(Debug, Clone)]
struct Node<T> where T: Clone + Debug + PartialOrd {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: Clone + Debug + PartialOrd {
    fn new(elt: T) -> Self {
        Node {
            element: elt,
            next: None,
        }
    }
    // 返回最后一个节点
    fn last(&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            return node.last();
        }
        self
    }
    // 根据索引下标查找节点
    fn search(&mut self, n: usize, i: usize) -> Option<&mut Self> {
        // 坐标相同时返回
        if n == i {
            return Some(self);
        } else {
            // 递归调用 所有权机制循环难以处理
            if let Some(ref mut node) = self.next {
                return node.search(n + 1, i);
            }
        }
        None
    }
}

// 合并有序 Node 递归实现
impl<T> Node<T> where T: Clone + Debug + PartialOrd {
    fn merge(n: Option<Box<Node<T>>>, m: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
        match (n, m) {
            (Some(x), None) => {
                Some(x)
            }
            (None, Some(y)) => {
                Some(y)
            }
            (Some(x), Some(y)) => {
                if x.element > y.element {
                    Some(Box::from(
                        Node {
                            element: y.element,
                            next: Node::merge(Some(x), y.next),
                        }
                    ))
                } else {
                    Some(Box::from(
                        Node {
                            element: x.element,
                            next: Node::merge(x.next, Some(y)),
                        }
                    ))
                }
            }
            (_, _) => {
                None
            }
        }
    }
}

// 单链表 head 头节点 len 长度
#[derive(Debug, Clone)]
pub struct Linked<T> where T: Clone + Debug + PartialOrd {
    head: Option<Box<Node<T>>>,
    len: usize,
}

// Vec => Linked 转换
impl<T> From<Vec<T>> for Linked<T> where T: Clone + Debug + PartialOrd {
    fn from(v: Vec<T>) -> Self {
        let mut l = Linked::new();
        for i in v {
            l.push(i)
        }
        l
    }
}

// Linked => Vec 转换
impl<T> Into<Vec<T>> for Linked<T> where T: Clone + Debug + PartialOrd {
    fn into(mut self) -> Vec<T> {
        let mut v = Vec::new();
        for i in 0..self.len {
            if let Some(n) = self.index(i) {
                v.push(n);
            }
        }
        v
    }
}

impl<T> Linked<T> where T: Clone + Debug + PartialOrd {
    pub fn new() -> Self {
        Linked {
            head: None,
            len: 0,
        }
    }
    // 反转
    pub fn reverse(&mut self) {
        // head 不为 None 进行反转处理
        if let Some(_) = self.head {
            // 从 head 的下一个节点开始，因已经判断 Some 则 unwrap 无风险
            let mut node = self.head.as_mut().unwrap().next.take();
            // 节点 不为 None 一直循环
            while node.is_some() {
                // take 所有权 给 temp
                let mut temp = node.take().unwrap();
                // 下一节点 指向 node
                node = temp.next;
                // head
                temp.next = self.head.take();
                self.head = Some(temp);
            }
        }
    }
    // 末尾压入节点
    pub fn push(&mut self, n: T) {
        // 根据 head 判断插入方式
        match self.head {
            None => {
                self.head = Some(Box::from(Node::new(n)));
            }
            Some(ref mut head) => {
                let last = head.last();
                last.next = Some(Box::from(Node::new(n)));
            }
        }
        self.len += 1;
    }
    // 根据索引下标删除节点
    pub fn remove(&mut self, i: usize) -> Option<T> {
        if let Some(ref mut head) = self.head {
            // 处理索引为 0 的情况
            if i == 0 {
                let node = head.element.clone();
                self.head = head.next.take();
                self.len -= 1;
                return Some(node);
            } else {
                // 处理其他情况
                if let Some(prev) = head.search(0, i - 1) {
                    // 临时变量 next 存储节点
                    if let Some(next) = prev.next.take() {
                        // prev = next => next
                        prev.next = next.next;
                        self.len -= 1;
                        return Some(next.element.clone());
                    }
                }
            }
        }
        None
    }
    // 索引插入节点
    pub fn insert(&mut self, i: usize, n: T) -> Option<()> {
        let mut new = Node::new(n);
        if let Some(ref mut head) = self.head {
            // 索引为 0 相当于链表头插入
            if i == 0 {
                // 通过 take 取出 head 内的值
                new.next = self.head.take();
                self.head = Some(Box::from(new));
            } else {
                if let Some(prev) = head.search(0, i - 1) {
                    if let Some(next) = prev.next.take() {
                        new.next = Option::from(next);
                        prev.next = Some(Box::from(new));
                    }
                }
            }
            self.len += 1;
        }
        None
    }
    // 末尾删除节点
    pub fn pop(&mut self) -> Option<T> {
        if let Some(ref mut head) = self.head {
            // 处理只有一个节点的情况
            if self.len == 1 {
                let last = head.element.clone();
                self.head = head.next.take();
                self.len -= 1;
                return Some(last);
            }
            // 处理其他情况
            let last = head.last().element.clone();
            if let Some(prev) = head.search(0, self.len - 2) {
                prev.next = None;
                self.len -= 1;
                return Some(last);
            }
        }
        None
    }
    // 根据索引下标查找节点
    pub fn index(&mut self, i: usize) -> Option<T> {
        if self.len > i {
            if let Some(ref mut head) = self.head {
                if let Some(elt) = head.search(0, i) {
                    return Some(elt.element.clone());
                }
            }
        }
        None
    }
    // 获取长度
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Linked<T> where T: Clone + Debug + PartialOrd {
    // 合并有序列表
    pub fn merge_order(n: Linked<T>, m: Linked<T>) -> Option<Linked<T>> {
        if let Some(node) = Node::merge(n.head, m.head) {
            // 调用 Node::merge
            return Some(Linked {
                head: Some(node),
                // 合并长度
                len: n.len + m.len,
            });
        }
        None
    }
}