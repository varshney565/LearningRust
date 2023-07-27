use crate::ownership_functions;

pub fn ownership_concept(){
    /*
    * Rules : 
    * 1 ) Each value in Rust has a variable that's called its owner.
    * 2 ) There can onlt be one owner at a time.
    * 3 ) Once the owner goes out of scope value will be dropped.
    */
    
    {
        let s : &str = "shivam"; //value in stack
        println!("value of string : {s}");
        //once the scope goes over we can't use 's' anymore
    }

    {
        let s : String = String::from("Hello");  //allocated on the heap
        println!("value of string : {s}");
        //once scope goes out rust compiler will automatically deallocate memory for us
    }


    let x = 2;
    let _y = x;  //it is a copy 

    let a = "string";
    let _b = a; //it is a copy not move

    //we can't change 'a' here because we are not allowed to change the string literals.
    //and also strings are stored in the form of bytes in the memory so we even can't access the 
    //value at any given index.

    let p = String::from("YES");
    let q = p; //move
    //now "YES" ownership is transferred to 'q'
    let mut _r = q.clone();
    /*
     *  if we do p.clone() we can't do it because 'p' is no more available.
     *  once we have shifted the ownership that variale is of no use.
     */

     // println!("String p : {}",p); ---> it will give error because p is no more accessable.
     ownership_functions :: ownership_fun();
}