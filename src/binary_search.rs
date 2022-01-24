// 二分查找
// 针对一个有序的数据集合，类似分治思想通过数次与区间元素对比，将查找区间减半直至查找到元素或者区间被缩小为0
// 时间复杂度O(logn)
// 递推公式：search(start...end,n) = search(start...end/2,n)
// 终止条件：start == end / middle = 0 / len = 1
// 局限性：数据需要是顺序结构，存放数据要有序，数据量太小和太大都不适合
pub fn binary_search<T>(v: &[T], n: T) -> Option<T> where T: PartialEq + PartialOrd + Clone {
    let len = v.len();
    if len == 0 {
        return None;
    }
    if len == 1 {
        if v[0] == n {
            return Some(v[0].clone());
        }
    }
    let middle = len / 2;
    if middle == 0 {
        return None;
    }
    if v[middle] > n {
        binary_search(&v[..middle], n);
    } else if v[middle] < n {
        binary_search(&v[middle..], n);
    } else {
        return Some(v[middle].clone());
    }
    None
}