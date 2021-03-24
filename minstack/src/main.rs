fn main() {
    println!("Hello, world!");
}

struct MinStack {
    val: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack { val: vec![] }
    }

    fn push(&mut self, val: i32) {
        self.val.push(val);
    }

    fn pop(&mut self) {
        self.val.pop();
    }

    fn top(&self) -> i32 {
        *self.val.iter().last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.val.iter().min().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn minstack_test() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(-3, stack.get_min());
        stack.pop();
        assert_eq!(0, stack.top());
        assert_eq!(-2, stack.get_min());
    }
}
