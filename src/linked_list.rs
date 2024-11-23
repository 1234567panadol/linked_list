// an un-rusty implementation of linked list

use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T> {
    val: T,
    next: NonNull<Node<T>>,
}

pub struct Pointer<'a, T> {
    ptr: NonNull<Node<T>>,
    index: usize,
    list: &'a UnsafeCell<List<T>>,
}

pub struct List<T> {
    begin: NonNull<Node<T>>,
    end: NonNull<Node<T>>,
    size: usize,
}

pub struct IntoIter<T> {
    list: List<T>,
}

pub struct Iter<'a, T> {
    ptr: NonNull<Node<T>>,
    end: NonNull<Node<T>>,
    _marker: PhantomData<&'a T>
}

pub struct IterMut<'a, T> {
    ptr: NonNull<Node<T>>,
    end: NonNull<Node<T>>,
    _marker: PhantomData<&'a mut T>
}

impl<'a, T> Pointer<'a, T> {
    fn new(ptr: NonNull<Node<T>>, list: &'a UnsafeCell<List<T>>) -> Self {
        Pointer { ptr, list, index: 0 }
    }
    pub fn push(&mut self, val: T) -> Pointer<'a, T> {
        unsafe {
            let prev = self.ptr;
            let node = Box::leak(Box::new(Node { val, next: (*prev.as_ptr()).next }));
            (*prev.as_ptr()).next = NonNull::new_unchecked(node);
            self.ptr = (*prev.as_ptr()).next;
            self.index += 1;
            Pointer::new(prev, self.list)
        }
    }
}

impl<'a, T> Drop for Pointer<'a, T> {
    fn drop(&mut self) {
        unsafe { 
            (*self.list.get()).end = self.ptr;
            (*self.list.get()).size += self.index;
        }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        unsafe { 
            let node = Box::leak(Box::new_uninit().assume_init());
            let ptr = NonNull::new_unchecked(node);
            List { begin: ptr, end: ptr, size: 0 }
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn front(&self) -> Option<&T> {
        if self.begin != self.end {
            unsafe { Some(&(*self.begin.as_ptr()).val) }
        } else {
            None
        }
    }

    pub fn back(&self) -> Option<&T> {
        if self.begin != self.end {
            unsafe { Some(&(*self.end.as_ptr()).val) }
        } else {
            None
        }
    }

    pub fn push_back(&mut self, val: T) {
        Pointer::new(self.end, UnsafeCell::from_mut(self)).push(val);
    }

    pub fn push_front(&mut self, val: T) {
        unsafe {
            (*self.begin.as_ptr()).val = val;
            let node: &mut Node<T> = Box::leak(Box::new_uninit().assume_init());
            node.next = self.begin;
            self.begin = NonNull::new_unchecked(node);
            self.size += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.begin != self.end {
            unsafe {
                let boxed = Box::from_raw(self.begin.as_ptr());
                self.begin = boxed.next;
                Some(boxed.val)
            }
        } else {
            None
        }
    }

    pub fn release(&mut self) -> Pointer<T> {
        Pointer::new(self.end, UnsafeCell::from_mut(self))
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { ptr: self.begin, end: self.end, _marker: PhantomData }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { ptr: self.begin, end: self.end, _marker: PhantomData }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        unsafe {
            while let Some(_) = self.pop_front() { }
            let _ = Box::from_raw(self.begin.as_ptr());
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }
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

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.ptr != self.end {
            unsafe {
                self.ptr = (*self.ptr.as_ptr()).next;
                Some(&mut (*self.ptr.as_ptr()).val)
            }
        } else {
            None
        }
    }
}
