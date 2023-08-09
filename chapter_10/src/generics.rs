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
struct Point<T,P> where T : Copy{
    x : T,
    y : P
}

impl<T,P> Point<T,P> where T : Copy{
    fn x(&self) -> T{
        self.x
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

    let p1 = Point{x : 1,y : 2.3};
    println!("point : {:?}",p1);
    println!("calling method : {}",p1.x());
}