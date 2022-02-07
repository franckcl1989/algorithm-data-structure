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

// 平衡二叉查找树：满足二叉查找树得前提下保证左右子树得高度差在一定范围内
// 红黑树
// 1. 根节点是黑色
// 2. 每个叶子节点都是黑色得空节点、叶子节点不存储数据
// 3. 任何相邻得节点不能同时为红色、红节点被黑节点隔开
// 4. 每个节点，从该节点到达其可达叶子节点得所有路径都包含相同数目得黑色节点
// 左旋：将关注节点置为左子节点（左旋后树依旧是一颗二叉查找树）
// 右旋：将关注节点置为右子节点（右旋后树依旧是一颗二叉查找树）

// 插入平衡：插入节点必须是红色得并且新插入得节点都放在叶子节点上
// 1. 如果插入节点得父节点是黑色、无需处理满足红黑树定义
// 2. 如果插入节点是根节点，将颜色设置为黑色即可
// 3. 其他情况需要通过左旋、右旋改变颜色使其继续满足红黑树定义
// CASE1:关注节点A 叔叔节点D是红色
// - 将关注节点A得父节点B、叔叔节点D设置为黑色
// - 将关注节点A祖父节点C设置为红色
// - 将关注节点设置为祖父节点C
// - 跳转CASE2或CASE3
// CASE2:关注节点A 叔叔节点D是黑色，A是其父节点B得右子节点
// - 关注节点设置为父节点B
// - 围绕B左旋
// - 跳转CASE3
// CASE3:关注节点A 叔叔节点D是黑色，A是其父节点B得左子节点
// - 围绕A得祖父节点C进行右旋
// - 将父节点B与兄弟节点颜色互换
// - 调整结束

// 删除平衡：针对删除节点初步调整、针对关注节点进行二次调整
// 初步调整
// CASE1:删除节点A且A只有一个子节点B
// - 删除节点A，子节点B放在A得位置
// - 将节点B设置为黑色
// - 调整结束、不需要二次调整
// CASE2:删除节点A且A有两个非空子节点并且后继节点是A得右子节点C
// - 删除节点A，右子节点C放在A得位置
// - 将C设置为与A相同得颜色
// - 将关注节点设置为C得右子节点D
// CASE3:删除节点A且A右两个非空子节点并且后继节点不是A得右子节点
// - 找到后继节点D并删除
// - 将A替换为D
// - 将D设置为A相同颜色
// - 将关注节点设置为C
// 二次调整
// CASE1:关注节点A，兄弟节点C为红色
// - 围绕A得父节点B左旋
// - 关注节点A和祖父节点C交换颜色
// CASE2:关注节点A，兄弟节点C为黑色并且C得左右子节点D、E为黑色
// - C设置为红色
// - 关注节点设置为父节点B
// CASE3:关注节点A，兄弟节点C为黑色并且C得左子节点为红色，C得右子节点为黑色
// - 围绕C右旋
// - 将C和D交换颜色
// - 跳转CASE4
// CASE4:关注节点A，兄弟节点C为黑色并且C得右子节点为红色
// - 围绕A得父节点B左旋
// - 将C设置为与B相同颜色
// - 将B设置为黑色
// - 将A得叔叔节点E设置为黑色
// - 调整结束
// 实现有点复杂暂时没写出来，查了一下Rust for Linux有相关实现 回头学习一下再自己写
// https://github.com/Rust-for-Linux/linux/blob/rust/rust/kernel/rbtree.rs

// 递归树
// 在进行时间复杂度分析时如果使用递推公式会涉及非常复杂的数学推导，借助递归树可以相对简单得来进行时间复杂度分析
// 递归是将大问题分解为小问题求解，把求解得过程化成图其实是一颗树
// 有些代码适合用递推公式来进行分析，比如归并排序时间复杂度、快速排序最好时间复杂度，有些适合递归树分析，比如快速排序平均时间复杂度