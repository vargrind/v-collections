//! terribly unsafe, circularly linked list
//! supports a variety of standard operations

use core::iter;
use core::mem;
use core::ptr;

pub struct Circular<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    value: T,
    next: *mut Node<T>,
    prev: *mut Node<T>,
}

#[derive(Clone, Copy)]
pub struct Iteration<'a, T> {
    first: Option<&'a Node<T>>,
    last: Option<&'a Node<T>>,
}

pub struct MutableIteration<'a, T> {
    first: Option<&'a mut Node<T>>,
    last: Option<&'a mut Node<T>>,
}

pub struct MovedIteration<T>(Circular<T>);

impl<T> Circular<T> {
    pub fn new() -> Self {
        Circular {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    /// add something to tail
    pub fn push(&mut self, what: T) {
        unsafe {
            let created = Box::into_raw(Box::new(Node {
                value: what,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            // list empty
            if (self.head.is_null()) {
                self.head = created;
                self.tail = created;
                (*created).next = created;
                (*created).prev = created;
            }
            // link to existing
            else {
                (*self.head).prev = created;
                (*self.tail).next = created;
                (*created).next = self.head;
                (*created).prev = self.tail;
                self.tail = created;
            }
        }
    }

    /// remove something from tail
    pub fn pop(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        } else if self.tail == self.head {
            unsafe {
                let raw = self.tail;
                self.tail = ptr::null_mut();
                self.head = ptr::null_mut();
                return Some(Box::from_raw(raw).value);
            }
        } else {
            unsafe {
                let raw = self.tail;
                self.tail = (*raw).prev;
                (*(*raw).prev).next = (*raw).next;
                (*(*raw).next).prev = (*raw).prev;
                (*raw).next = ptr::null_mut();
                (*raw).prev = ptr::null_mut();
                return Some(Box::from_raw(raw).value);
            }
        }
    }

    /// add something to head
    pub fn unshift(&mut self, what: T) {
        unsafe {
            let created = Box::into_raw(Box::new(Node {
                value: what,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            }));
            // list empty
            if (self.head.is_null()) {
                self.head = created;
                self.tail = created;
                (*created).next = created;
                (*created).prev = created;
            }
            // link to existing
            else {
                (*self.head).prev = created;
                (*self.tail).next = created;
                (*created).next = self.head;
                (*created).prev = self.tail;
                self.head = created;
            }
        }
    }

    /// remove something from head
    pub fn shift(&mut self) -> Option<T> {
        if self.tail.is_null() {
            return None;
        } else if self.tail == self.head {
            unsafe {
                let raw = self.head;
                self.tail = ptr::null_mut();
                self.head = ptr::null_mut();
                return Some(Box::from_raw(raw).value);
            }
        } else {
            unsafe {
                let raw = self.head;
                self.head = (*raw).next;
                (*(*raw).prev).next = (*raw).next;
                (*(*raw).next).prev = (*raw).prev;
                (*raw).next = ptr::null_mut();
                (*raw).prev = ptr::null_mut();
                return Some(Box::from_raw(raw).value);
            }
        }
    }

    /// peeks the front element as a shared reference
    pub fn front(&self) -> Option<&T> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            return Some(&(*self.head).value);
        }
    }

    /// peeks the front element as a mutable reference
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.head.is_null() {
            return None;
        }
        unsafe {
            return Some(&mut (*self.head).value);
        }
    }

    /// peeks the rear element as a shared reference
    pub fn rear(&self) -> Option<&T> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            return Some(&mut (*self.tail).value);
        }
    }

    /// peeks the rear element as a mutable reference
    pub fn rear_mut(&mut self) -> Option<&mut T> {
        if self.tail.is_null() {
            return None;
        }
        unsafe {
            return Some(&mut (*self.tail).value);
        }
    }

    pub fn into_iter(self) -> MovedIteration<T> {
        MovedIteration(self)
    }

    pub fn iter(&self) -> Iteration<T> {
        unsafe {
            Iteration {
                first: self.head.as_ref(),
                last: self.tail.as_ref(),
            }
        }
    }

    pub fn iter_mut(&self) -> MutableIteration<T> {
        unsafe {
            MutableIteration {
                first: self.head.as_mut(),
                last: self.tail.as_mut(),
            }
        }
    }
}

impl<T> Drop for Circular<T> {
    fn drop(&mut self) {
        // lazy drop: recursively move things out of scope and drop them.
        while let Some(_) = self.pop() {}
    }
}

impl<'a, T> iter::Iterator for Iteration<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {}
}

impl<'a, T> iter::Iterator for MutableIteration<'a, T> {}

impl<T> iter::Iterator for MovedIteration<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gauntlet() {
        let list: Circular<i32> = Circular::new();
    }

    fn test_list() -> Circular<i32> {
        let v = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 10];
        let mut built = Circular::new();
        v.into_iter().for_each(|x| built.push(x));
        built
    }
}
