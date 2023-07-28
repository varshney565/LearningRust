#[derive(Debug)]

struct Rectangle{
    width : i32,
    height : i32
}

fn area(r : &Rectangle) -> i32 {
    return r.width*r.height;
}

pub fn r(){
    let rect = Rectangle { width: (12), height: (10) };
    let ar = area(&rect);
    println!("area of the rectangle {:?} is : {}",rect,ar);
}
