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
    // recover_error::use_it();
}
