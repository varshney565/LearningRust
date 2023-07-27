mod slicing;
mod borrowing;
mod ownership;
mod ownership_functions;
mod dangling_pointer;
fn main() {
    ownership::ownership_concept();
    borrowing::borrowing_concept();
    slicing::method1();
    slicing::method2();
}