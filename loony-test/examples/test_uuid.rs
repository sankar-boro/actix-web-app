use uuid::{Uuid, v1::{Timestamp, Context}};
use chrono::prelude::*;

fn utc_time() {
    let local: DateTime<Utc> = Utc::now(); 
    println!("{:?}", local);
}

fn local_time() {
    let utc: DateTime<Local> = Local::now(); 
    println!("{:?}", utc);
}

fn main() {
    utc_time();
    local_time();
}