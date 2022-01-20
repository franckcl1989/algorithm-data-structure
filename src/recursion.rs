use std::collections::{HashMap, HashSet};

// 数据结构和算法最难的两个知识点：动态规划、递归
// 很多数据结构和算法需要用到递归：DFS深度优先搜索、前中后序二叉树遍历等
// 递归即 发送为"递归" 返回为"归"，满足三个要素即可使用递归
// 1. 可以拆解成相同类型子问题 2. 子问题除数据规模不同思路完全一致 3. 存在终止条件
// 例子：确认座位问题 已知前面座位为 n - 1 求自己座位 n 是几号
// f(n) = f(n-1) + 1 f(1) = 1
pub fn f(n: u32) -> u32 {
    // 退出条件
    if n == 1 {
        return 1;
    }
    // 子问题 f(n) = f(n - 1) + 1
    return f(n - 1) + 1;
}

// 递归的关键是写出递推公式和找到终止条件
// 例子：n个台阶，每次可以跨1个或者2个，求一共有多少种走法（这个题之前刷leetcode刷到过#70）
// 解析：每次跨1或者2一共是两种走法，f(n)总方法 = f(n-1)先跨1 + f(n-2)先跨2，终止条件为f(1) && f(2) 剩1阶（1种走法）或剩2阶（2种走法）
// climbing_stairs(n)就是f(n)
pub fn climbing_stairs_v1(n: u32) -> u32 {
    // 终止条件1 f(1)
    if n == 1 {
        return 1;
    }
    // 终止条件2 f(2)
    if n == 2 {
        return 2;
    }
    // f(n - 1) + f(n - 2) Rust是表达式语言 最后这个不用写 return xxx;
    climbing_stairs_v1(n - 1) + climbing_stairs_v1(n - 2)
}

// 递归关键是抽象为递推公式，不要去模拟计算机的层层调用，人脑模拟难度太大
// 写递归代码要注重两点：1. 递归次数谨防爆栈 2. 警惕重复计算提高性能
// 改善重复计算f(5) = f(5 - 1)`4` + f(5 - 2)`3` f(4) = f(4 - 1)`3` + f(4 - 2)`2`
pub fn climbing_stairs_v2(n: u32, m: &mut HashMap<u32, u32>) -> u32 {
    // 终止条件1 f(1)
    if n == 1 {
        return 1;
    }
    // 终止条件2 f(2)
    if n == 2 {
        return 2;
    }

    // 处理重复子问题
    if let Some(x) = m.get(&n) {
        return *x;
    }

    // f(n - 1) + f(n - 2) Rust是表达式语言 最后这个不用写 return xxx;
    let ret = climbing_stairs_v2(n - 1, m) + climbing_stairs_v2(n - 2, m);
    m.insert(n, ret);
    ret
}

// 理论上递归代码都可以改成循环方式，并且很多编译器也会针对递归循环这种做一些优化（汇编可能会是一样的）
// 找椅子
pub fn find_seat(n: u32) -> u32 {
    let mut m = 0;
    for i in 0..n {
        m += 1;
    }
    m
}

// 爬楼梯
pub fn climbing_stairs_v3(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }
    if n == 2 {
        return 2;
    }
    let mut ret = 0;
    let mut one = 1;
    let mut two = 2;
    for i in 3..n + 1 {
        ret = one + two;
        one = two;
        two = ret;
    }
    ret
}
// 不论是递归还是循环都会有一些问题要考虑 例如"环"需要检测预防，还有调用长度的问题
// 编写代码要充分考虑，控制好可能出现的副作用做好边界检查和测试