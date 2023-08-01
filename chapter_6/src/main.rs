mod enumUse;
mod option;
mod pattern;
mod if_let;
enum demo{
    // A(String),
    b(i32),
    c(String,String),
    // e(String,String),
    f(String)
}

/*
 *                    8    8    8    8    8    8
 * String           ---- ---- ----
 * String,String    ---- ---- ---- ---- ---- ----
 * 
 */

/*
 *              
 * Sting,String     ---- ---- ---- ---- ---- ----
 * String           ---- ---- ---- 
 * String           ---- ---- ---- 
 *  
 * In that case we have to keep a tag which will tell us about 
 * which perticular value is currently is being uses because we can't
 * distingues btw 2nd and 3rd 
 */
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
    if_let::last();
}