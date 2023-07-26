fn main() {
    //ownership 

    /*
     * Rules : 
     * 1 ) Each value in Rust has a variable that's called its owner.
     * 2 ) There can onlt be one owner at a time.
     * 3 ) Once the owner goes out of scope value will be dropped.
     */

//example 1 : 
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

//example 2 : 
    let x = 2;
    let _y = x;  //it is a copy 

    let a = "string";
    let _b = a; //it is a copy not move
    //we can't change 'a' here because we are not allowed to change the string literals.

    let p = String::from("YES");
    let q = p; //move
    //now "YES" ownership is transferred to 'q'
    let mut _r = q.clone();
    /*
     *  if we do p.clone() we can't do it because 'p' is no more available.
     *  once we have shifted the ownership that variale is of no use.
     */
    
    // println!("String p : {}",p); ---> it will give error because p is no more accessable.

    test(q);
    // println!("String : {}",q); --> we can't do it because the ownership got shifted to the variable in test::st
    // and as this variable got out of scope so memeory also released.  


    //NOTE
    //1 : We can have any number of immutable references but can have only one mutable reference.
    
}


//create a function

fn test(st : String) {
    println!("string : {}",st);
}
