pub fn borrowing_concept(){
    
    //point 1
    //borrowing means creating a reference variable instead of passing the actual object.
    let mut s = String :: from("Temp");
    checker(&s);
    println!("String : {} ---> Surprize !! ",s);


    //Point2
    //1 : We can have any number of immutable references or only one mutable reference.
    //2 : we can't use the previous immutable reference once we have declared a mutuable referece
    //    or made any changes in the original string.

    let s1 = &s;
    let s2 =  &s;
    //do
    //anything
    //you 
    //wanna
    //do
    println!("Accessing Strings using immutable reference : {},{}",s1,s2);

    s.push_str(" String"); 
    println!("String after it has changed : {s}");

    //here we are changing the string so we can't use
    //previous references so if we using them it will through error
    // println!("{s1}");
    let s3 = &s;
    println!("String ---> {}",s3);
}

fn checker(st : &String){
    //here referece is coming not the actual object
    println!("Done checking String : {}!!",st);
    //so after that line memory won't get cleaned because we are using the reference
}