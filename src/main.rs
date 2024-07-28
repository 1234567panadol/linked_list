#![feature(new_uninit)]
use std::{marker::PhantomData, ptr::NonNull};
use std::iter::{Iterator, IntoIterator};
mod test;

//an un-rusty implementation of linked lists

struct Node<T> {
    val: T,
    next: NonNull<Node<T>>,
}

struct List<T> {
    begin: NonNull<Node<T>>,
    end: NonNull<Node<T>>,
    size: usize,
    _marker: PhantomData<Box<Node<T>>>,
}

struct Iter<'a, T> {
    ptr: NonNull<Node<T>>,
    end: NonNull<Node<T>>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr != self.end {
            unsafe {
                self.ptr = (*self.ptr.as_ptr()).next;
                Some(&(*self.ptr.as_ptr()).val)
            }
        } else {
            None
        }
    }
}

struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

trait BoxToNonNull {
    type Item;
    fn new_nonnull(x: Self::Item) -> NonNull<Self::Item>;
    unsafe fn new_uninit_nonnull() -> NonNull<Self::Item>;
}

impl<T> BoxToNonNull for Box<T> {
    type Item = T;
    
    fn new_nonnull(x: T) -> NonNull<T> {
        let boxed = Box::new(x);
        unsafe { NonNull::new_unchecked(Box::leak(boxed)) }
    }

    unsafe fn new_uninit_nonnull() -> NonNull<T> {
        let boxed = Box::new_uninit();
        NonNull::new_unchecked(Box::leak(boxed.assume_init()))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() { }
        let _ = unsafe { Box::from_raw(self.begin.as_ptr()) }; 
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        unsafe {
            let node: NonNull<Node<T>> = Box::new_uninit_nonnull();
            (*node.as_ptr()).next = node;
            List { begin: node, end: node, size: 0, _marker: PhantomData}
        }
    }

    pub fn push_front(&mut self, val: T) {
        unsafe {
            (*self.begin.as_ptr()).val = val;
            let node: NonNull<Node<T>> = Box::new_uninit_nonnull();
            (*node.as_ptr()).next = self.begin;
            self.begin = node;
            self.size += 1;
        }
    }

    pub fn push_back(&mut self, val: T) {
        unsafe {
            let node: NonNull<Node<T>> = Box::new_uninit_nonnull();
            (*node.as_ptr()).val = val;
            (*node.as_ptr()).next = node;
            (*self.end.as_ptr()).next = node;
            self.end = (*self.end.as_ptr()).next;
            self.size += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.begin != self.end {
            unsafe {
                self.begin = (*self.begin.as_ptr()).next;
                self.size -= 1;
                Some(Box::from_raw(self.begin.as_ptr()).val)
            }
        } else {
            None
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.begin != self.end {
            unsafe { Some(&self.begin.as_ref().next.as_ref().val) }
        } else {
            None
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.begin != self.end {
            unsafe { Some(&mut self.begin.as_mut().next.as_mut().val) }
        } else {
            None
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.begin != self.end {
            unsafe { Some(&self.end.as_ref().val) }
        } else {
            None
        }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.begin != self.end {
            unsafe { Some(&mut self.end.as_mut().val) }
        } else {
            None
        }
    }
    
    pub fn append_front(&mut self, other: &mut Self) {
        unsafe { 
            other.end.as_mut().next = self.begin.as_ref().next;
            self.begin = other.begin;
            self.size += other.size;
            other.end = other.begin;
            other.size = 0;
        }
    }

    pub fn append_back(&mut self, other: &mut Self) {
        unsafe { 
            self.end.as_mut().next = other.begin.as_ref().next;
            self.end = other.end;
            self.size += other.size;
            other.end = other.begin;
            other.size = 0;
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { ptr: self.begin, end: self.end, _marker: PhantomData }
    }
}

fn main() {
    let mut list: List<i32> = List::new();

    for i in 0..100 {
        list.push_front(i);
        list.push_back(i)
    }
    
    for i in list {
        println!("{i}");
    }

}
