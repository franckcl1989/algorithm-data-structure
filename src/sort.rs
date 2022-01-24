// 排序算法众多最经典最常用的：冒泡、插入、选择、归并、快速、计数、基数、桶几种
// 其中冒泡、插入、选择基于比较，时间复杂度为O(n^2)
// 快速、归并基于比较，时间复杂度为O(nlogn)
// 桶、计数、基数不是基于比较，时间复杂度为O(n)

// 分析排序算法一般从以下几个方向衡量
// 执行效率
// 1.最好情况、最坏情况、平均情况时间复杂度：因为有些算法会区分另外实际情况数据可能接近有序也有完全无序，需要知道在不同情况下的性能表现
// 2.时间复杂度的系数、常数、低阶：实际开发中数据规模n大概率不会很大所以在同阶算法比对要考虑系数、常数、低阶
// 3.比较次数与交换次数：基于比较的算法执行过程会涉及元素比较和交换/移动，分析时应该把比较、交换/移动次数考虑进来

// 内存消耗
// 1.空间复杂度衡量内存消耗
// 2.原地排序：特质空间复杂度O(1)的算法

// 稳定性
// 1.如果排序中存在相等的元素，经过排序之后相等元素的先后顺序保持不变则为稳定
// 2.实际业务中排序的不是基础数据类型而是复合数据类型，稳定算法适合类似业务

use std::collections::HashSet;
use std::fmt::Debug;
use std::io::Take;
use std::ops::Range;

// 冒泡排序
// 比较相邻两个元素，不满足比较关系进行互换，一次冒泡至少使一个元素移动到应该的位置重复n次完成n个数据的排序
// 空间复杂度O(1)属于原地排序
// 稳定算法，只有不同才会发生交换
// 时间复杂度最好O(n)，最坏O(n^2)，平均O(n^2)
pub fn bubble<T>(v: &mut Vec<T>) where T: PartialEq + PartialOrd + Copy {
    // 外层循环次数根据元素个数定义
    for i in 0..v.len() {
        // 内存循环次数根据比较次数定义（比较次数 = 元素个数 - 当前循环次数 - 1）
        for j in 0..v.len() - i - 1 {
            if v[j] > v[j + 1] {
                let temp = v[j + 1];
                v[j + 1] = v[j];
                v[j] = temp;
            }
        }
    }
}

// 不严格的分析方式：有序度、逆序度表示具备有序、逆序元素对的个数(依次跟后续的比较)
// 2 4 3 1 5 6 有序度(2 4)(2 3)(2 5)(2 6)(4 5)(4 6)(3 5)(3 6)(1 5)(1 6)(5 6)
// 完全有序的叫做满有序度 n * (n - 1) / 2，逆序度 = 满有序度 - 有序度

// 插入排序
// 将数据分成已排序、未排序区间，初始已排序区间只有第一个元素，依次从未排序区间取元素在已排序区间进行比较找到合适位置插入
// 空间复杂度O(1)属于原地排序
// 稳定算法，只有不同才会发生交换
// 时间复杂度最好O(n)，最坏O(n^2)，平均O(n^2)
pub fn insert<T>(v: &mut Vec<T>) where T: PartialEq + PartialOrd + Copy {
    // 外层循环次数根据初始未排序区间定义
    for i in 1..v.len() {
        // 内存循环次数根据已排序区间定义
        for j in 0..i {
            if v[i] < v[i - j] {
                v[i - j + 1] = v[i - j];
            } else {
                break;
            }
        }
        v[i - 1] = v[i];
    }
}

// 选择排序
// 将数据分成已排序、未排序区间，从未排序区间找到最小元素将其放置已排序区间的末尾，这里已排序区间初始为无
// 空间复杂度O(1)属于原地排序
// 不稳定算法，每次都会寻找最小值并进行交换
// 时间复杂度最好O(n)，最坏O(n^2)，平均O(n^2)
pub fn selection<T>(v: &mut Vec<T>) where T: PartialEq + PartialOrd + Copy {
    // 外层循环次数根据初始未排序区间定义（所有元素）
    for i in 0..v.len() {
        // 内层循环次数根据未排序区间定义（注意跟外层初始的区别）
        let mut min = i;
        for j in i..v.len() {
            if v[j] < v[min] {
                min = j;
            }
        }
        let temp = v[i];
        v[i] = v[min];
        v[min] = temp;
    }
}

// 归并排序、快速排序适合大规模数据排序相交冒泡、插入、选择更常用，归并与快排都用到了分治思想
// 归并排序
// 将数组从中间分成前后两部分对前后两部分分别进行排序，再将排序好的部分合并在一起
// 归并排序使用分治思想将一个大问题分解为小的子问题解决，分治是处理思想、递归是编程技巧
//         给下标从p到r数组排序 子问题 q = (p + r)/2 中间位置
// 递推公式：merge_sort(p...r) = merge(merge_sort(p...q),merge_sort(q+1...r))
// 终止条件：p >= r
// 排序函数主要是实现递推公式即：排序的结果start...end -> 合并(start...end/前半，start...end/后半)
// 稳定算法：优先处理前半段
// 空间复杂度:O(n)
// 时间复杂度：O(nlogn)
pub fn merge_sort<T>(v: &mut [T]) where T: PartialEq + PartialOrd + Copy + Debug {
    // 中间位置
    let middle = v.len() / 2;
    // 退出条件（无法拆分子问题）
    if middle == 0 {
        return;
    }
    // p...q
    merge_sort(&mut v[..middle]);
    // q + 1...r
    merge_sort(&mut v[middle..]);
    // 合并
    merge(v, middle);
}

// 合并函数是排序主要逻辑，比较元素大小push临时数组、扫描剩余元素push临时数组，临时数组复制给原数组
pub fn merge<T>(v: &mut [T], n: usize) where T: PartialEq + PartialOrd + Copy + Debug {
    let len = v.len();
    // 申请空间长度一致的临时数组
    let mut temp: Vec<T> = Vec::with_capacity(v.len());
    // 前半区间下标（从0开始）
    let mut i = 0;
    // 后半区间下标（从中间位置开始）
    let mut j = n;
    // 循环比较
    for _ in 0..len {
        // 前半或者后半区间元素扫描到退出循环
        if i == n || j == len {
            break;
        }
        // 前半后半依次比较，小的存入临时数组，下标前进一位
        if v[i] > v[j] {
            temp.push(v[j]);
            j += 1;
        } else {
            temp.push(v[i]);
            i += 1;
        }
    }
    // 扫描前半剩余元素
    if i < n {
        for x in i..n {
            temp.push(v[x]);
        }
    }
    // 扫描后半剩余元素
    if j < len {
        for x in j..len {
            temp.push(v[x]);
        }
    }
    // 将临时数组的数据依次写入
    for m in 0..len {
        v[m] = temp[m];
    }
}

// 快速排序
// 从数组中选取一个分区点，将小于分区点的数据放置在分区的前半部分，大于分区点的数据放置在后半部分，重复这个过程
// 递推公式：quick_sort(p...r) = quick_sort(p...q-1),quick_sort(q+1...r))
// 终止条件：p >= r
// 排序函数主要是实现递推公式即：排序的结果start...end -> (start...分区点/前半，分区点...end/后半)
// 不稳定算法：会发生元素顺序交换
// 空间复杂度:O(1)
// 时间复杂度：O(nlogn)
pub fn quick_sort<T>(v: &mut [T]) where T: PartialEq + PartialOrd + Copy + Debug {
    // 分区没数据长度为0
    if v.len() == 0 {
        return;
    }
    // 分区点
    let mut p = v.len() - 1;
    // 退出条件（无法拆分子问题）
    if p == 0 {
        return;
    }
    // 分区处理
    partition(v, &mut p);
    // 分区前半段
    quick_sort(&mut v[..p]);
    // 分区后半段
    quick_sort(&mut v[p + 1..]);
}

// 分区处理函数是排序主要逻辑，依次扫描元素与分区进行比较，比分区点小的不做处理比分区大的与分区交换（因为分区用的是最后一个元素）
pub fn partition<T>(v: &mut [T], p: &mut usize) where T: PartialEq + PartialOrd + Copy + Debug {
    // 分区点数据
    let pivot = v[*p];
    // 指针A下标i
    let mut i = 0;
    // 指针B下标j，循环从 0..分区点
    for j in 0..*p {
        // 比较指针B与分区点数据，小的话交换A B指针数据并且A指针前进一步
        // 这相当于将小于分区点的元素放置最左侧
        // 指针A 相当于临时分区点
        if v[j] < pivot {
            let temp = v[j];
            v[j] = v[i];
            v[i] = temp;
            i += 1;
        }
    }
    // 交换指针A 与分区点
    let temp = v[*p];
    v[*p] = v[i];
    v[i] = temp;
    *p = i;
}

// 桶排序：线性排序、非元素比较排序，适合用于外部排序（即数据存放在磁盘存储）数据量巨大内存有限的情况
// 将数据分配到有序的桶中，将每个桶数据排序然后合并每个桶
// 需要数据具备天然区间可以很容易划分桶、桶与桶之间要有天然顺序合并桶不需要再进行排序、数据分布需要比较平均（每个桶）
pub fn bucket(v: Vec<i32>) -> Vec<i32> {
    let mut b = vec![vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![], vec![]];
    for i in v {
        match i {
            0..=10 => {
                b[0].push(i);
            }
            11..=20 => {
                b[1].push(i);
            }
            21..=30 => {
                b[2].push(i);
            }
            31..=40 => {
                b[3].push(i);
            }
            41..=50 => {
                b[4].push(i);
            }
            51..=60 => {
                b[5].push(i);
            }
            61..=70 => {
                b[6].push(i);
            }
            71..=80 => {
                b[7].push(i);
            }
            81..=90 => {
                b[8].push(i);
            }
            91..=100 => {
                b[9].push(i);
            }
            _ => {}
        }
    }
    for x in b.iter_mut() {
        quick_sort(&mut x[..]);
    }
    let mut v = b[0].clone();
    for y in 1..b.len() - 1 {
        v.append(&mut b[y])
    }
    v
}

// 计数排序
// 计数排序属于桶排序的一种特殊情况，当所处理的数据范围不大的时候可以直接按照数据规模分桶，省掉了桶内排序的时间
pub fn count(v: Vec<i32>) -> Vec<i32> {
    let len = v.len();
    // a 保存分数对应考生个数，下标为分数 值为个数
    let mut c = vec![0; 6];
    for i in v.clone() {
        c[i as usize] += 1;
    }
    // 依次累计每个元素，此时下标对应分数 元素对应小于等于下标（分数）的个数
    for i in 1..c.len() {
        c[i] = c[i - 1] + c[i]
    }
    let mut r = vec![0; len];
    // 循环 v 元素，从 c 中获取下标为 v 的元素，c[v[i]] 代表小于等于 v 的数量（则r[x] = r[c[v[i]] - 1]]）
    for i in 0..len - 1 {
        let i = len - 1 - i;
        let index = (c[v[i] as usize] - 1) as usize;
        r[index] = v[i];
        c[v[i] as usize] -= 1;
    }
    r
}

// 基数排序
// 可以独立分成'位'按位比较、位直接有递进关系，位的数据范围要小可以使用线性算法排序
// 借助稳定排序算法，基数排序适合手机号字符串之类的排序，如果遇到数据非等长需要按位补0