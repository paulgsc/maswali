
use std::collections::HashMap;
use std::ptr;

struct Node {
    key: i32,
    value: i32,
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
}

pub struct LRUCache {
    capacity: usize,
    cache: HashMap<i32, *mut Node>,
    head: *mut Node,
    tail: *mut Node,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Box::into_raw(Box::new(Node {
            key: 0,
            value: 0,
            prev: None,
            next: None,
        }));
        let tail = Box::into_raw(Box::new(Node {
            key: 0,
            value: 0,
            prev: None,
            next: None,
        }));

        unsafe {
            (*head).next = Some(tail);
            (*tail).prev = Some(head);
        }

        LRUCache {
            capacity: capacity as usize,
            cache: HashMap::new(),
            head,
            tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&node_ptr) = self.cache.get(&key) {
            unsafe {
                self.move_to_front(node_ptr);
                (*node_ptr).value
            }
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(&node_ptr) = self.cache.get(&key) {
            unsafe {
                (*node_ptr).value = value;
                self.move_to_front(node_ptr);
            }
        } else {
            if self.cache.len() >= self.capacity {
                self.evict();
            }
            let new_node = Box::into_raw(Box::new(Node {
                key,
                value,
                prev: None,
                next: None,
            }));
            self.cache.insert(key, new_node);
            self.add_to_front(new_node);
        }
    }

    fn move_to_front(&mut self, node_ptr: *mut Node) {
        unsafe {
            self.remove(node_ptr);
            self.add_to_front(node_ptr);
        }
    }

    fn add_to_front(&mut self, node_ptr: *mut Node) {
        unsafe {
            let next_node = (*self.head).next.unwrap();
            (*node_ptr).prev = Some(self.head);
            (*node_ptr).next = Some(next_node);
            (*self.head).next = Some(node_ptr);
            (*next_node).prev = Some(node_ptr);
        }
    }

    fn remove(&mut self, node_ptr: *mut Node) {
        unsafe {
            let prev_node = (*node_ptr).prev.unwrap();
            let next_node = (*node_ptr).next.unwrap();
            (*prev_node).next = Some(next_node);
            (*next_node).prev = Some(prev_node);
        }
    }

    fn evict(&mut self) {
        unsafe {
            let lru_node = (*self.tail).prev.unwrap();
            self.remove(lru_node);
            self.cache.remove(&(*lru_node).key);
            Box::from_raw(lru_node);
        }
    }
}

fn main() {
    let mut lru_cache = LRUCache::new(2);

    lru_cache.put(1, 1);
    lru_cache.put(2, 2);
    println!("{}", lru_cache.get(1));
    lru_cache.put(3, 3);
    println!("{}", lru_cache.get(2));
    lru_cache.put(4, 4);
    println!("{}", lru_cache.get(1));
    println!("{}", lru_cache.get(3));
    println!("{}", lru_cache.get(4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut lru_cache = LRUCache::new(2);

        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);

        lru_cache_put(3, 3);
        assert_eq!(lru_cache.get(2), -1);

        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }
}






