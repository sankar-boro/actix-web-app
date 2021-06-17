fn main() {
    let mut x = String::from("sankar");
    let y = &mut x;
    do_something(&x);
}    

fn do_something(a: &String) {

}