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

pub struct Iteration<T> {
    first: *const Node<T>,
    last: *const Node<T>,
}

pub struct MutableIteration<T> {
    first: *mut Node<T>,
    last: *mut Node<T>,
}

impl<T> Circular<T> {
    pub fn new() -> Self {
        Circular {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }
    
    pub fn push(&mut self, what: T) {
        
    }

    pub fn pop(&mut self, what: T) {
        
    }

    pub fn shift(&mut self, what: T) {

    }

    pub fn unshift(&mut self, what: T) {
        
    }

    /// peeks the front element as a shared reference
    pub fn front(&self) -> Option<&T> {
        Option::None
    }

    /// peeks the front element as a mutable reference
    pub fn front_mut(&mut self) -> Option<&mut T> {
        Option::None

    }

    /// peeks the rear element as a shared reference
    pub fn rear(&self) -> Option<&T> {
        Option::None
    }

    /// peeks the rear element as a mutable reference
    pub fn rear_mut(&mut self) -> Option<&mut T> {
        Option::None
    }
    
}

// impl<T> Iterator for Iteration<T> {
    
//     fn next (&mut self) -> Option<Self::Item> {
//         Option::None
//     }
// }

// impl<T> Clone for Iteration<T> {

// }

// impl<T> Iterator for MutableIteration<T> {
    
// }

// impl<T> Clone for MutableIteration<T> {
    
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn gauntlet() {
        let list: Circular<i32> = Circular::new();
    }
}