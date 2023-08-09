use core::panic;
mod recover_error;
fn main() {
    a();
}

fn a(){
    b();
}

fn b(){
    c();
}

fn c(){
    let check = false;
    if check {
        panic!("Error !");
    }

    let res = recover_error::error_propogation();
    match(res){
        Ok(s) => println!("{s}"),
        Err(error) => println!("Error")
    }
    //those below lines will be executed even if there is any error in the above lines.
    recover_error::use_it();
    println!("Yes");
}
