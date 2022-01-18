use std::fmt::Debug;

// 定义节点
#[derive(Debug, Clone)]
struct Node<T> where T: Clone + Debug {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: Clone + Debug {
    fn new(elt: T) -> Self {
        Node {
            element: elt,
            next: None,
        }
    }
    fn last(&mut self) -> &mut Self {
        if let Some(ref mut node) = self.next {
            return node.last();
        }
        self
    }

    fn index(&mut self, n: usize, i: usize) -> Option<&mut Self> {
        if n == i {
            return Some(self);
        }
        if let Some(ref mut node) = self.next {
            node.index(n + 1, i);
        }
        None
    }
}

// 单链表 head 头节点 len 长度
#[derive(Debug, Clone)]
pub struct Linked<T> where T: Clone + Debug {
    head: Option<Node<T>>,
    len: usize,
}

impl<T> Linked<T> where T: Clone + Debug {
    pub fn new() -> Self {
        Linked {
            head: None,
            len: 0,
        }
    }

    pub fn push(&mut self, n: T) {
        match self.head {
            None => {
                self.head = Some(Node::new(n));
            }
            Some(ref mut head) => {
                let last = head.last();
                last.next = Some(Box::from(Node::new(n)));
            }
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(ref mut head) = self.head {
            let last = head.last().element.clone();
            if let Some(prev) = head.index(0, self.len - 2) {
                prev.next = None;
                self.len -= 1;
                return Some(last);
            }
        }
        None
    }

    pub fn index(&mut self, i: usize) -> Option<T> {
        if let Some(ref mut head) = self.head {
            if i < self.len {
                if let Some(node) = head.index(0, i) {
                    return Some(node.element.clone());
                }
            }
        }
        None
    }
}