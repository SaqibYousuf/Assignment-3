use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(||{
        for i in 0..20{
            println!("spawn thread : {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    for i in 0..10 {
        println!("main thread 1: {}",i);
        thread::sleep(Duration::from_millis(1));
    }

    for y in 10..20 {
        println!("main thread 2: {}",y);
        thread::sleep(Duration::from_millis(1));
    }

    
    handle.join().unwrap();
}
