use std::fmt::Debug;

// 二分查找
// 针对一个有序的数据集合，类似分治思想通过数次与区间元素对比，将查找区间减半直至查找到元素或者区间被缩小为0
// 时间复杂度O(logn)
// 递推公式：search(start...end,n) = search(start...end/2,n)
// 终止条件：start == end / middle = 0 / len = 1
// 局限性：数据需要是顺序结构，存放数据要有序，数据量太小和太大都不适合
pub fn binary_search<T>(v: &[T], n: T) -> Option<T> where T: PartialEq + PartialOrd + Clone + Debug {
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
    if v[middle] == n {
        return Some(v[middle].clone());
    }
    if v[middle] > n {
        return binary_search(&v[..middle], n);
    } else if v[middle] < n {
        return binary_search(&v[middle..], n);
    } else {
        return Some(v[middle].clone());
    }
}

// 变体1 第一个匹配元素
pub fn binary_search_v1<T>(v: &[T], n: T) -> Option<T> where T: PartialEq + PartialOrd + Clone {
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
        return binary_search_v1(&v[..middle], n);
    } else if v[middle] < n {
        return binary_search_v1(&v[middle..], n);
    } else {
        // 判断前面元素是否相等
        if len > 1 {
            if v[middle - 1] == n {
                binary_search_v1(&v[..middle], n);
            }
        }
        return Some(v[middle].clone());
    }
}

// 变种2 最后一个匹配元素
pub fn binary_search_v2<T>(v: &[T], n: T) -> Option<T> where T: PartialEq + PartialOrd + Clone {
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
        return binary_search_v2(&v[..middle], n);
    } else if v[middle] < n {
        return binary_search_v2(&v[middle..], n);
    } else {
        // 判断后面元素是否相等
        if middle == 0 && len > 1 {
            if v[middle + 1] == n {
                binary_search_v2(&v[..middle], n);
            }
        }
        return Some(v[middle].clone());
    }
}

// 变种3 第一个大于等于匹配元素的值
pub fn binary_search_v3<T>(v: &[T], n: T) -> Option<T> where T: PartialEq + PartialOrd + Clone {
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
    // 没有相同值时取下一个
    if middle == 1 && len == 2 {
        return Some(v[1].clone());
    }
    if v[middle] > n {
        return binary_search_v3(&v[..middle], n);
    } else if v[middle] < n {
        return binary_search_v3(&v[middle..], n);
    } else {
        return Some(v[middle].clone());
    }
}

// 变种3 第一个小于等于匹配元素的值
pub fn binary_search_v4<T>(v: &[T], n: T) -> Option<T> where T: PartialEq + PartialOrd + Clone {
    let len = v.len();
    if len == 0 {
        return None;
    }
    if len == 1 {
        return Some(v[0].clone());
    }

    let middle = len / 2;
    if middle == 0 {
        return None;
    }

    if v[middle] > n {
        return binary_search_v4(&v[..middle], n);
    } else if v[middle] < n {
        // 没有相同值时取下一个
        if v[len - 1] < n {
            return Some(v[len - 1].clone());
        }
        return binary_search_v4(&v[middle..], n);
    } else {
        return Some(v[middle].clone());
    }
}