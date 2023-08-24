pub fn left_time() {
    let s1 = "abc".to_string();
    let s2 = "defgh";
    let s3 = longest(s1.as_str(),s2);
    println!("Largest String : {}",s3);
}

// ❌
// fn largest(x : &str,y : &str) -> &str {

// }

//💡 
//what <'a> it means is : returned value will take the lifetime of the variable
//that has smaller lifetime
fn longest<'a>(x : &'a str,y : &'a str) -> &'a str{
    if x.len() < y.len() {
        y
    }else{
        x
    }
}

/*
 * Note : we are getting the error because function's return type contains a borrowed value
 *        the signature does not say whether it is borrowed from `x` or `y`.
 */
