
use std::ptr;

#[derive(Debug)]
pub struct Node<T>
where T: Copy {
    value: T,
    next: *mut Node<T>,
}

impl<T> Node<T>
where T: Copy {
    pub fn new(value: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node { value, next: ptr::null_mut() }))
    }

    pub fn value(&self) -> T {
        self.value
    }

    pub fn next(&self) -> *mut Node<T> {
        self.next
    }
}

#[derive(Debug)]
pub struct JankedList<T>
where T: Copy {
    len: usize,
    head: *mut Node<T>,
}

impl<T> JankedList<T>
where T: Copy {
    pub fn new() -> JankedList<T> {
        JankedList {
            len: 0,
            head: ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn push_back(&mut self, value: T) {
        self.len += 1;

        let new_node = Node::new(value);

        if self.is_empty() {
            self.head = new_node;
            return;
        }

        unsafe {
            let mut curr = self.head;
            while !(*curr).next.is_null() {
                curr = (*curr).next;
            }
            (*curr).next = new_node;
        }
    }

    pub fn push_front(&mut self, value: T) {
        self.len += 1;

        let new_node = Node::new(value);

        unsafe {
            (*new_node).next = self.head;
        }
        self.head = new_node;
    }
}

impl<T> Default for JankedList<T>
where T: Copy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_creation() {
        let list: JankedList<i32> = JankedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn list_push() {
        let mut list: JankedList<i32> = JankedList::new();
        list.push_back(1);
        list.push_front(-1);
    }
}
