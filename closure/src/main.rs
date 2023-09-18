use std::{thread, time::Duration};
fn main() {
    let mut a = vec![1,2,3];
    let mut closure = || {
        a.push(4);
        println!("{:?}",a);
    };
    closure();
    closure();

    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(10));
    });
    handle.join().unwrap();
}