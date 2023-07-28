#[derive(Debug)]
struct Rectangle{
    width : i32,
    height : i32
}

//implementation block will contain all the methods that are realated to rectangle
impl Rectangle{
    fn area(&self)-> i32{
        return self.width*self.height;
    }

    fn can_hold(&self,other : &Rectangle) -> bool{
        if(self.width >= other.width && self.height >= other.height){
            return true;
        }
        return false;
    }
}

impl Rectangle{
    fn Square(size : i32)->Rectangle{
        Rectangle { width: (size), height: (size) }
    }
}

pub fn run(){
    let r = Rectangle{width : 12,height : 14};
    let ar = r.area();
    println!("Rectangle : {:?} area is : {}",r,ar);

    //Let's create a square
    let sq = Rectangle::Square(12);
    println!("Square is : {:?}",sq);
}