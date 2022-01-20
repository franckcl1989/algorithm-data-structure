// 栈结构 容量 元素数量 底层存储 特点后进先出
#[derive(Debug)]
pub struct Stack<T> {
    pub len: usize,
    pub count: usize,
    inner: Vec<T>,
}

// 实现入栈 出栈
impl<T> Stack<T> {
    pub fn new(n: usize) -> Self {
        Stack {
            len: n,
            count: 0,
            inner: vec![],
        }
    }
    // 入栈
    pub fn push(&mut self, n: T) -> Option<()> {
        if (self.count + 1) <= self.len {
            self.inner.push(n);
            self.count += 1;
            return Some(());
        }
        None
    }
    // 出栈
    pub fn pop(&mut self) -> Option<T> {
        if self.count >= 1 {
            self.count -= 1;
            return self.inner.pop();
        }
        None
    }
}

// 模拟浏览器
pub struct Browser<T> {
    forward: Stack<T>,
    backward: Stack<T>,
}

// 为了方便采用实现 Copy 类型
impl<T> Browser<T> where T: Copy {
    pub fn new() -> Self {
        Browser {
            forward: Stack::new(10),
            backward: Stack::new(10),
        }
    }
    // 浏览 n 压入 forward
    pub fn browse(&mut self, n: T) -> Option<()> {
        self.forward.push(n)
    }
    // 前进 backward 出栈 n 压入 forward
    pub fn forward(&mut self) -> Option<T> {
        if let Some(page) = self.backward.pop() {
            self.forward.push(page);
            return self.current();
        }
        None
    }
    // 后退 forward 出栈 n 压入 backward
    pub fn backward(&mut self) -> Option<T> {
        if let Some(page) = self.forward.pop() {
            self.backward.push(page);
            return self.current();
        }
        None
    }
    // 当前应返回页面
    fn current(&mut self) -> Option<T> {
        // forward 不为空时 返回最后一个页面
        if self.forward.count != 0 {
            return Some(self.forward.inner[self.forward.count - 1]);
        }
        None
    }
}

// 括号匹配
pub fn brackets_match(s: &str) -> bool {
    let mut v = Stack::new(50);
    let s = s.chars();
    for i in s {
        match i {
            '(' | '[' | '{' => {
                v.push(i);
            }
            ')' => {
                if let Some(x) = v.pop() {
                    if x != '(' {
                        return false;
                    }
                }
            }
            ']' => {
                if let Some(x) = v.pop() {
                    if x != '[' {
                        return false;
                    }
                }
            }
            '}' => {
                if let Some(x) = v.pop() {
                    if x != '{' {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }
    if v.count == 0 {
        return true;
    }
    false
}

// 计算表达式
pub fn calculate(s: &str) -> Option<i32> {
    let s = s.chars();
    let mut v1 = Stack::new(20);
    let mut v2 = Stack::new(20);
    // 循环表达式
    for i in s {
        match i {
            '+' | '-' | '*' | '/' => {
                if v2.count == 0 {
                    v2.push(i);
                } else {
                    let y = v2.inner[v2.count - 1];
                    if ord(i, y) {
                        if let Some(b) = v2.pop() {
                            if let Some(a2) = v1.pop() {
                                if let Some(a1) = v1.pop() {
                                    if let Some(n) = cal(a1, a2, b) {
                                        v1.push(n);
                                        v2.push(i);
                                    }
                                }
                            }
                        }
                    } else {
                        v2.push(i);
                    }
                }
            }
            _ => {
                if let Ok(x) = i.to_string().parse() {
                    v1.push(x);
                }
            }
        }
    }
    // 计算剩余表达式元素
    for _ in 0..v1.count {
        if let Some(b) = v2.pop() {
            if let Some(a2) = v1.pop() {
                if let Some(a1) = v1.pop() {
                    if let Some(n) = cal(a1, a2, b) {
                        v1.push(n);
                    }
                }
            }
        }
    }
    // 返回计算结果
    v1.pop()
}

// 比较优先级 实际感觉用 Enum 实现符号的 Ord 和 Eq 会更好
// 优先级低或者相同返回 true 优先级高返回 false
fn ord(x: char, y: char) -> bool {
    match (x, y) {
        ('+', '-') | ('-', '+') | ('*', '/') | ('/', '*') => {
            true
        }
        ('+', '*') | ('+', '/') | ('-', '*') | ('-', '/') => {
            true
        }
        (_, _) => {
            false
        }
    }
}

// 计算转换 char 为计算操作符返回结果
fn cal(x: i32, y: i32, z: char) -> Option<i32> {
    match z {
        '+' => {
            Some(x + y)
        }
        '-' => {
            Some(x - y)
        }
        '*' => {
            Some(x * y)
        }
        '/' => {
            Some(x / y)
        }
        _ => {
            None
        }
    }
}