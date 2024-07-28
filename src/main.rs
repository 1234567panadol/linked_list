#![feature(new_uninit)]
mod linked_list;

fn main() {
    let mut list: linked_list::List<i32> = linked_list::List::new();

    for i in 0..100 {
        list.push_front(i);
        list.push_back(i)
    }
    
    for i in list {
        println!("{i}");
    }
}
