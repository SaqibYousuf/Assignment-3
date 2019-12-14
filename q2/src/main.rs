fn name<T:FnMut()>(mut y:T){
    y();
}
fn main() {
    let mut x = String::from("Saqib");
    let z = || x.push_str(" Yousuf"); 
    name(z);
    println!("Student Name : {}",x);

    let mut a = String::from("Muhammad");
    let b = || a.push_str(" Yousuf");
    name(b);
    println!("Father Name : {}",a);
}