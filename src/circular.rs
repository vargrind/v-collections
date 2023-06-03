//! unsafe, circularly linked list
//! supports a variety of standard operations

use core::iter;
use core::ptr;
use core::mem;

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

pub struct MovedIteration<T> (Circular<T>);

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
            let created = Box::into_raw(
                Box::new(
                    Node {
                        value: what,
                        next: ptr::null_mut(),
                        prev: ptr::null_mut(),
                    }
                )
            );
            // list empty
            if(self.head.is_null()){
                self.head = created;
                self.tail = created;
                (*created).next = created;
                (*created).prev = created;
            }
            // link to existing
            else{
                (*self.head).prev = created;
                (*self.tail).next = created;
                (*created).next = self.head;
                (*created).prev = self.tail;
                self.tail = created;
            }
        }
    }

    /// remove something from tail
    pub fn pop(&mut self) -> Option<T>{
        
    }

    /// add something to head
    pub fn unshift(&mut self, what: T) {
        
    }

    /// remove something from head
    pub fn shift(&mut self) -> Option<T>{
        
    }

    /// peeks the front element as a shared reference
    pub fn front(&self) -> Option<&T> {

    }

    /// peeks the front element as a mutable reference
    pub fn front_mut(&mut self) -> Option<&mut T> {


    }

    /// peeks the rear element as a shared reference
    pub fn rear(&self) -> Option<&T> {

    }

    /// peeks the rear element as a mutable reference
    pub fn rear_mut(&mut self) -> Option<&mut T> {

    }
    
}

impl<T> Drop for Circular<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<'a, T> Iterator for Iteration<'a, T> {
    type Item = &'a T;

    fn next (&mut self) -> Option<Self::Item> {
        
    }
}

impl<'a, T> Iterator for MutableIteration<'a, T> {
    
}

impl<T> Iterator for MovedIteration<T> {
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
}