/**
 * Problem Statement:
 * write a function that takes a string of words separated by spaces and returns the first word it 
 * finds in that string. If the function doesn’t find a space in the string, the whole string must be
 * one word, so the entire string should be returned.
 * 
 */

//method 1
pub fn method1(){
    let mut s = String::from("Hello Shivam Varshney !!");
    let ind = go(&s);
    //now what if i cleared that string
    s.clear();
    //now what if i try to access the length of the string it will be stealed data.
    println!("Length of string \"{}\" : {}",s,ind);
}

fn go(st : &String)-> usize {
    let mut i = 0;
    for element in st.chars(){
        if element == ' ' {
            return i;
        }
        i = i+1;
    }
    return st.len();
}   

//method 2

pub fn method2(){
    let mut s = String :: from("Hello Shivam Varshney !!");
    let r = goo(&s);  //pointer to heap memory but as a immutable reference 
    println!("{}",r);
    s.clear(); //at this point I have cleared the String, mean i have dome mutable operation.

    //what if i try to access 'r' at this point ???
    //it will give me error, WHY ????
    //it is just because 'r' is immutable reference

    // println!("{}",r);---> this line will give error
}

fn goo(st : &String) -> &str{
    let mut i = 0;
    for element in st.chars(){
        if element == ' ' {
            return &st[..i];
        }
        i = i+1;
    }
    return &st[..];
}