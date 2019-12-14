#![allow(unused)]
fn main() {
fn customer_order<F>(func: F)
    where F: FnOnce() -> String
{
        println!("Customer order: {}", func());
    let Review = String::from("I have enjoyed alot!");
    println!("Customer review: {}",Review);    
}

let y = String::from("Biryani");
let put_value_of_y = || y;
customer_order(put_value_of_y);

}
