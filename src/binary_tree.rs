// 树
// 根节点：没有父节点
// 兄弟节点：同一个父节点
// 叶子节点：没有子节点
// 高度：节点到叶子节点得最长路径
// 深度：根节点到节点所经历得边得个数
// 层数：深度+1

use std::fmt::Debug;

// 二叉树
// 每个节点最多有两个子节点，左子节点右子节点
// 满二叉树：叶子节点全在最底层并且除了叶子节点之外每个节点都有左右两个子节点
// 完全二叉树：叶子节点都在最底下两层，最后一层叶子节点都靠左排列并且除了最后一层其他层得节点个数都要达到最大
// 二叉树得存储方法：基于指针或引用得二叉链式存储、基于数组得顺序存储
// 链式存储法：每个节点3个字段 存储数据 存储左、右两个子节点得指针（大部分二叉树得实现方式）
// 顺序存储法：根节点存储下标 i = 1得位置，左子节点 2 * i = 2、右子节点 2 * i + 1 = 3以此类推
// 完全二叉树仅浪费下标为0得空间如果非完全浪费较多，堆其实是一种完全二叉树最常见得存储方式就是数组
// 二叉树遍历：前序遍历、中序遍历、后序遍历
// 前序遍历：对树中任意节点，先打印该节点、再打印左子树、最后打印右子树
// 中序遍历：对树中任意节点，先打印左子树、再打印该节点、最后打印右子树
// 后序遍历：对树中任意节点，先打印左子树、再打印右子树、最后打印该节点
#[derive(Debug)]
pub struct BinaryTree<T> where T: Debug {
    pub root: Option<()>,
    pub elt: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> where T: Debug {
    pub fn new(n: T) -> Self {
        BinaryTree {
            root: None,
            elt: n,
            left: None,
            right: None,
        }
    }
    // 前序遍历
    // pre(r) = print(r) -> pre(r.left) -> pre(r.right)
    pub fn pre_order(&mut self) {
        if let Some(_) = self.root {
            if self.left.is_none() && self.right.is_none() {
                return;
            }
            println!("{:?}", self.elt);
        } else {
            println!("{:?}", self.elt);
        }
        if let Some(ref mut left) = self.left {
            left.pre_order();
        }
        if let Some(ref mut right) = self.right {
            right.pre_order();
        }
    }
    // 中序遍历
    // in(r) = in(r.left) -> print(r) -> in(r.right)
    pub fn in_order(&mut self) {
        if let Some(_) = self.root {
            if self.left.is_none() && self.right.is_none() {
                return;
            }
        }
        if let Some(ref mut left) = self.left {
            left.in_order();
        }
        println!("{:?}", self.elt);
        if let Some(ref mut right) = self.right {
            right.in_order();
        }
    }
    // 后序遍历
    // post(r) = post(r.left) -> post(r.right) -> print(r)
    pub fn post_order(&mut self) {
        if let Some(_) = self.root {
            if self.left.is_none() && self.right.is_none() {
                return;
            }
        }
        if let Some(ref mut left) = self.left {
            left.post_order();
        }
        if let Some(ref mut right) = self.right {
            right.post_order();
        }
        println!("{:?}", self.elt);
    }
}

// 二叉查找树
// 树中任意一个节点左子树小于节点、右子树大于节点
type Node<T> = Option<Box<BinarySearchTree<T>>>;

#[derive(Debug, Clone)]
pub struct BinarySearchTree<T> {
    elt: Option<T>,
    left: Node<T>,
    right: Node<T>,
}

impl<T> BinarySearchTree<T> where T: Debug + Clone + Ord {
    // 初始均为None
    pub fn new() -> Self {
        BinarySearchTree {
            elt: None,
            left: None,
            right: None,
        }
    }
    // 删除数据
    fn delete(&mut self, prev: &mut BinarySearchTree<T>, n: T) {
        match self.elt.clone() {
            None => {}
            Some(e) => {
                // 数据匹配则返回
                if n == e {
                    if self.left.is_none() && self.right.is_none() {
                        prev.left = None;
                    }
                    if self.left.is_some() && self.right.is_none() {
                        prev.left = self.left.clone();
                    }
                    if self.right.is_some() && self.left.is_none() {
                        prev.right = self.right.clone();
                    }
                    if self.left.is_some() && self.right.is_some() {
                        let mut next = self.right.clone();
                        while next.is_some() {
                            next = next.take();
                        }
                        prev.left = next;
                        next = None;
                    }
                } else {
                    // 不匹配则根据大小寻找下一个节点
                    let next = if n < e { &mut self.left } else { &mut self.right }.take();
                    match next {
                        None => {}
                        // 节点不为None递归调用
                        Some(mut node) => {
                            node.delete(self, n)
                        }
                    }
                }
            }
        }
    }
    // 查找数据
    pub fn search(&mut self, n: T) -> Option<&T> {
        match &self.elt {
            None => {
                None
            }
            Some(e) => {
                // 数据匹配则返回
                if n == *e {
                    Some(e)
                } else {
                    // 不匹配则根据大小寻找下一个节点
                    let next = if n < *e { &mut self.left } else { &mut self.right };
                    match next {
                        None => {
                            None
                        }
                        // 节点不为None递归调用
                        Some(node) => {
                            node.search(n)
                        }
                    }
                }
            }
        }
    }
    // 插入数据
    pub fn push(&mut self, n: T) {
        match &self.elt {
            None => {
                // 头节点为None直接插入
                self.elt = Some(n);
            }
            // 否则根据大小继续寻找插入点
            Some(e) => {
                // 根据大小判断插入子节点
                let next = if n < *e { &mut self.left } else { &mut self.right };
                match next {
                    // 子节点为None插入
                    None => {
                        let mut node = BinarySearchTree::new();
                        node.push(n);
                        *next = Some(Box::new(node));
                    }
                    // 不为None递归调用
                    Some(ref mut node) => {
                        node.push(n);
                    }
                }
            }
        }
    }
}