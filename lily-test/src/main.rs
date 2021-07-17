use std::{thread, time};
use std::sync::{Arc, RwLock};

fn main() {
    let a = String::from("sankar");
    let b = Arc::new(a);
    let c = Arc::clone(&b);

    thread::spawn(move || {
       println!("{}", b); 
    });
    thread::spawn(move || {
       println!("{}", c); 
    });
}    