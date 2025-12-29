struct MyStack<T> {
    data: Vec<T>,
}

impl<T> MyStack<T> {
    /// 创建一个空栈
    fn new() -> Self {
        MyStack { data: Vec::new() }
    }

    /// 向栈顶压入一个元素
    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    /// 从栈顶弹出一个元素，如果栈为空则返回 `None`
    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.data.pop()
        }
    }

    /// 判断栈是否为空
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack() {
        let mut stack = MyStack::new();
        assert!(stack.is_empty());

        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert!(!stack.is_empty());

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }
}