use std::ptr::NonNull;

fn main() {
    let mut val = String::from("value");
    let ptr = NonNull::new(&mut val);
    unsafe {
        let a = (*ptr.unwrap().as_ptr()).len();
        let b = (*ptr.unwrap().as_ptr()).len();
        if a == b {

        }
    }
}