use std::ptr;

pub trait RawDroppable {
    fn raw_dump_drop(self);
}

impl<T> RawDroppable for *mut Node<T>
where
    T: Copy,
{
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    fn raw_dump_drop(self) {
        if !self.is_null() {
            unsafe {
                let _ = Box::from_raw(self);
            }
        }
    }
}

pub trait Linkable<T>
where
    T: Copy,
{
    fn contains(&mut self, target: T) -> bool;

    fn insert_at(&mut self, index: usize, value: T);

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn pop_head(&mut self) -> Option<T>;

    fn pop_tail(&mut self) -> Option<T>;

    fn push_back(&mut self, value: T);

    fn push_front(&mut self, value: T);

    fn remove_at(&mut self, index: usize) -> Option<T>;
}

pub trait Nodal<T>
where
    T: Copy,
{
    fn next(&self) -> *mut Node<T>;

    fn prev(&self) -> *mut Node<T>;

    fn value(&self) -> T;
}

#[derive(Debug, Clone, Copy)]
pub struct Node<T>
where
    T: Copy,
{
    next: *mut Node<T>,
    value: T,
}

impl<T> Node<T>
where
    T: Copy,
{
    #[rustfmt::skip]
    fn new(value: T) -> *mut Node<T> {
        Box::into_raw(Box::new(Node { value, next: ptr::null_mut() }))
    }
}

impl<T> Nodal<T> for Node<T>
where
    T: Copy,
{
    fn next(&self) -> *mut Node<T> {
        self.next
    }

    fn prev(&self) -> *mut Node<T> {
        ptr::null::<Node<T>>() as *mut Node<T>
    }

    fn value(&self) -> T {
        self.value
    }
}

impl<T> Default for Node<T>
where
    T: Copy + Default,
{
    fn default() -> Self {
        unsafe { *Node::new(T::default()) }
    }
}

#[derive(Debug)]
pub struct JankedList<T>
where
    T: Copy,
{
    head: *mut Node<T>,
    len: usize,
}

impl<T> JankedList<T>
where
    T: Copy,
{
    #[rustfmt::skip]
    fn new() -> JankedList<T> {
        JankedList { len: 0, head: ptr::null_mut() }
    }
}

impl<T> Linkable<T> for JankedList<T>
where
    T: Copy + PartialEq,
{
    fn contains(&mut self, target: T) -> bool {
        if self.is_empty() {
            return false;
        }

        unsafe {
            let mut curr = self.head;
            while !curr.is_null() {
                if (*curr).value == target {
                    return true;
                }
                curr = (*curr).next;
            }
        }
        false
    }

    fn insert_at(&mut self, index: usize, value: T) {
        if index > self.len {
            return;
        }

        self.len += 1;

        let new_node = Node::new(value);

        unsafe {
            if index == 0 {
                (*new_node).next = self.head;
                self.head = new_node;
                return;
            }

            let mut curr = self.head;
            for _ in 0..(index - 1) {
                if curr.is_null() {
                    new_node.raw_dump_drop();

                    self.len -= 1;
                    return;
                }
                curr = (*curr).next;
            }

            if !curr.is_null() {
                (*new_node).next = (*curr).next;
                (*curr).next = new_node;
            } else {
                new_node.raw_dump_drop();
                self.len -= 1;
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    fn len(&self) -> usize {
        self.len
    }

    fn pop_head(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;

        unsafe {
            if (*self.head).next.is_null() {
                let old_head = self.head;
                let value = (*old_head).value;
                self.head = ptr::null_mut();

                old_head.raw_dump_drop();

                return Some(value);
            }

            let mut curr = self.head;
            while !(*curr).next.is_null() && !(*(*curr).next).next.is_null() {
                curr = (*curr).next;
            }

            let tail = (*curr).next;
            let value = (*tail).value;
            (*curr).next = ptr::null_mut();

            tail.raw_dump_drop();

            Some(value)
        }
    }

    fn pop_tail(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;

        unsafe {
            let old_head = self.head;
            let value = (*old_head).value;
            self.head = (*old_head).next;

            old_head.raw_dump_drop();

            Some(value)
        }
    }

    fn push_back(&mut self, value: T) {
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

    fn push_front(&mut self, value: T) {
        self.len += 1;

        let new_node = Node::new(value);

        unsafe {
            (*new_node).next = self.head;
        }
        self.head = new_node;
    }

    fn remove_at(&mut self, index: usize) -> Option<T> {
        if self.is_empty() || index >= self.len {
            return None;
        }

        self.len -= 1;

        unsafe {
            if index == 0 {
                let old_head = self.head;
                let value = (*old_head).value;
                self.head = (*old_head).next;

                old_head.raw_dump_drop();

                return Some(value);
            }

            let mut curr = self.head;
            for _ in 0..(index - 1) {
                if curr.is_null() {
                    self.len += 1;
                    return None;
                }
                curr = (*curr).next;
            }

            if !curr.is_null() && !(*curr).next.is_null() {
                let to_remove = (*curr).next;
                let value = (*to_remove).value;
                (*curr).next = (*to_remove).next;

                to_remove.raw_dump_drop();

                Some(value)
            } else {
                self.len += 1;
                None
            }
        }
    }
}

impl<T> Default for JankedList<T>
where
    T: Copy,
{
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
    fn list_push_front() {
        let mut list: JankedList<i32> = JankedList::new();
        list.push_front(1);
        list.push_front(2);

        assert!(list.len() == 2);
    }

    #[test]
    fn list_push_back() {
        let mut list: JankedList<i32> = JankedList::new();
        list.push_back(1);
        list.push_back(2);

        assert!(list.len() == 2);
    }

    #[test]
    fn list_contains() {
        let mut list = JankedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(5);

        assert!(list.contains(5));
        assert!(!list.contains(4));
    }

    #[test]
    fn list_removal() {
        let mut list: JankedList<i32> = JankedList::new();
        list.push_back(1);
        list.push_front(-1);
        list.pop_head();

        assert!(list.len() == 1);
    }

    #[test]
    fn empty_check() {
        let list: JankedList<i32> = JankedList::new();

        assert!(list.is_empty())
    }

    #[test]
    fn head_pop() {
        let mut list = JankedList::new();
        list.push_back(10);
        list.push_front(100);

        assert!(list.pop_head() == Some(10))
    }

    #[test]
    fn tail_pop() {
        let mut list = JankedList::new();
        list.push_back(10);
        list.push_front(100);

        assert!(list.pop_tail() == Some(100))
    }

    #[test]
    fn insert_at() {
        let mut list = JankedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(5);
        list.insert_at(3, 4);

        assert!(list.contains(4));
        assert!(list.len() == 5)
    }

    #[test]
    fn remove_at() {
        let mut list = JankedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(5);
        list.insert_at(3, 4);

        assert!(list.remove_at(3) == Some(4));
        assert!(list.len() == 4)
    }

    #[test]
    fn dump() {
        let node = Node::new(10);
        node.raw_dump_drop();

        assert!(!node.is_null());
    }

    #[test]
    fn dump_via_pop() {
        let mut list = JankedList::new();
        list.push_back(10);
        let value = list.pop_head();

        assert!(value == Some(10));
        assert!(list.len() == 0);
        assert!(list.head.is_null());
    }

    #[test]
    fn volatility_testing() {
        let mut list = JankedList::new();

        assert!(list.is_empty());

        (0..10_000).for_each(|x| {
            list.push_back(x % 50);
            list.push_front(x % 50);
        });

        assert!(list.len == 20_000);

        (0..10_000).for_each(|x| {
            list.remove_at(x % 50);
        });

        assert!(list.len == 10_000);

        (0..10_000).for_each(|x| {
            list.insert_at(x % 50, 100);
        });

        assert!(list.len == 20_000);

        assert!(list.pop_tail() == Some(100));
    }
}
