mod enumUse;
mod option;
mod pattern;
enum demo{
    // A(String),
    c(String,String),
    d(i32),
    // e(String,String)
}
//struct implementation
struct demo2{
    Yes : i8, // 1 byte 
    Number : i32, //4 bytes
    Array : String, //24 bytes
    c : char, //1 byte,
    d : char,
    e : char
}

fn main() {
    let s = std::mem::size_of::<demo>();
    println!("Size : {s}");
    enumUse::Ip();
    enumUse::Ip();
    option::option();
    pattern::patmat();
}