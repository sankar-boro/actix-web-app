use std::{thread};
use std::sync::{Arc};

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