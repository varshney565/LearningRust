use std::fmt::Display;

use crate::traits::*;

//------------->normal syntax

// pub fn notify(item : &impl Summary){
//     println!("Breaking News !! {}",item.summarize());
// }

// pub fn notify1(item1 : &impl Summary,item2 : &impl Summary) {

// }

// pub fn notify2<>(item1 : &(impl Summary + Display),item2 : &(impl Summary + Display)) {

// }

//------------->Trait bound syntax

pub fn notify<T : Summary>(item : &T){
    println!("Breaking News !! {}",item.summarize());
}

pub fn notify1<T : Summary>(item1 : &T,item2 : &T) {

}

pub fn notify2<T : Summary + Display>(item : &T) {
    
}

//------------>Improving readability.

use core::fmt::Debug;

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}

//instead try this

fn some_function_better<T,U>(t :&T,u :&U) -> i32
    where T : Display+Clone,
          U : Clone+Debug
{
    1
}