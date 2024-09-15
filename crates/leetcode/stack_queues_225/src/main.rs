
use std::collections::VecDeque;

struct MyStack {
    queue1: VecDeque<i32>,
    queue2: VecDeque<i32>,
}

impl MyStack {

    fn new() -> Self {
        MyStack {
            queue1: VecDeque::new(),
            queue2: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue1.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        while self.queue1.len() > 1 {
            if let Some(val) = self.queue1.pop_front() {
                self.queue2.push_back(val);
            }
        }
        
        let top_element = self.queue1.pop_front().unwrap();

        std::mem::swap(&mut self.queue1, &mut self.queue2);

        top_element
    }

    fn top(&mut self) -> i32 {
        while self.queue1.len() > 1 {
            if let Some(val) = self.queue1.pop_front() {
                self.queue2.push_back(val);
            }
        }

        let top_element = *self.queue1.front().unwrap();

        if let Some(val) = self.queue1.pop_front() {
            self.queue2.push_back(val);
        }
        std::mem::swap(&mut self.queue1, &mut self.queue2);

        top_element
    }

    fn empty(&self) -> bool {
        self.queue1.is_empty()
    }
}

fn main() {
    let mut stack = MyStack::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("Top element {}", stack.top());
    println!("Popped element: {}", stack.pop());
    println!("Top element after pop: {}", stack.top());
    println!("Is stack stack empty? {}", stack.empty());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_top() {
        let mut stack = MyStack::new();
        stack.push(10);
        stack.push(20);
        assert_eq!(stack.top(), 20, "Top element should be 20 after two pushes");

        stack.push(30);
        assert_eq!(stack.top(), 30, "Top element should be 30 after pushing 30");
    }

    #[test]
    fn test_pop() {
        let mut stack = MyStack::new();
        stack.push(10);
        stack.push(20);
        stack.push(30);

        assert_eq!(stack.pop(), 30, "Popped element should be 30");
        assert_eq!(stack.top(), 20, "Top element should be 20 after popping 30");

        assert_eq!(stack.pop(), 20, "Popped element should be 20");
        assert_eq!(stack.top(), 10, "Top element should be 10 after popping 20");

        assert_eq!(stack.pop(), 10, "Popped element should be 10");
    }

    #[test]
    fn test_empty() {
        let mut stack = MyStack::new();

        assert!(stack.empty(), "Stack should be empty initially");

        stack.push(1);
        assert!(!stack.empty(), "Stack should not be emptyp after one push");

        stack.pop();
        assert!(stack.empty(), "Stack should be empty after popping the only element");
    }

    #[test]
    fn test_mixed_operations() {
        let mut stack = MyStack::new();
        assert!(stack.empty(), "Stack should be empty initially");

        stack.push(5);
        stack.push(15);
        stack.push(25);
        assert_eq!(stack.top(), 25, "Top element should be 25");

        assert_eq!(stack.pop(), 25, "Popped elmeent should be 25");
        assert_eq!(stack.top(), 15, "Top elmeent should be 15");

        stack.push(35);
        assert_eq!(stack.top(), 35, "Top element should be 35");

        assert_eq!(stack.pop(), 35, "Popped elmeent should be 35");
        assert_eq!(stack.top(), 15, "Top elmeent should be 15");
        assert_eq!(stack.pop(), 15, "Popped elmeent should be 15");

        assert!(!stack.empty(), "Stack should not be empty");
        assert_eq!(stack.pop(), 5, "Popped element should be 5");
        assert!(stack.empty(), "Stack should be empty after all pops");
    }
}
