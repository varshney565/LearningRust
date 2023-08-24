/**
 * fn get_largest(arr : &Vec<i32>) -> &i32{
 *      let mut max_ = &arr[0];
 *      for num in arr {
 *          max_ = std::cmp::max(max_,num);
 *      }
 *      max_
 * }
 * 
 * Method : 1
 */


fn get_largest<T:std::cmp::Ord>(arr : &Vec<T>) -> &T {
    let mut max_ = &arr[0];
    for num in arr {
        max_ = std::cmp::max(max_,num);
    }
    max_
}

#[derive(Debug)]
struct Point<T,P> where T : Copy,P : Copy{
    x : T,
    y : P   
}

impl<T,P> Point<T,P> where T : Copy,P : Copy{
    fn x(&self) -> &T{
        &self.x
    }

    fn mixup<U,W>(&self,other : &Point<U,W>) -> Point<T,W> where U : Copy,W : Copy{
        Point{
            x:self.x,
            y:other.y
        }
    }
}

//or return the reference

// struct Point{
//     x : i32,
//     y : f32
// }

// impl Point{
//     fn x(&self) -> i32{
//         self.x
//     }
// }

enum Option<T>{
    Some(T),
    None
}

enum Result<T,E>{
    Ok(T),
    Err(E)
}

pub fn generics_use(){
    let k = vec![1,200,3,4,5,100];
    let ans = get_largest(&k);
    println!("Largest element of vector : {:?} is {}",k,ans);

    //struct 
    let p1 = Point{x : "shivam",y : 2.3};
    let p2 = Point{x : 45 , y : "varshney"};
    let _point3 = p1.mixup(&p2);
    println!("point : {:?}",p1);
    println!("calling method x : {}",p1.x());
    println!("point : {:?}",p2);
    println!("point : {:?}",_point3);
}


//at compile time rust will automatically replace the <Type> with 
//corrosponding types that are used in the program