#![feature(unsafe_cell_from_mut)]

mod linked_list;

use linked_list::*;

fn main() {
    let mut list = List::<i32>::new();
    list.push_back(10);
    {
        let mut ptr = list.release();
        let mut ptr2 = ptr.push(69);
        ptr.push(69);
        ptr2.push(59);
    }
    for (i, elem) in list.iter().enumerate() {
        println!("element {}: {}", i, elem);
    }
    println!("size: {}", list.size());
}