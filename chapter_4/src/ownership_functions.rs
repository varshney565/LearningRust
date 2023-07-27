pub fn ownership_fun(){
    let s = String :: from("Shivam");
    take_ownership(s);
    //at that point we can't use 's' any more because its ownership has gone 
    // println!("{s}");
}

fn take_ownership(st : String){
    println!("String is : {}",st);
    //now it's time to clear the memory of heap that st is pointing because st is no more accessible
}