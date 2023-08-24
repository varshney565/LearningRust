pub fn left_time() {
    let str1 = "abc".to_string();
    let str3 : &str;
    {
        let str2 = "mo".to_string();
        str3 = longest(str1.as_str(), str2.as_str());
        println!("Largest String : {}",str3);
    }

    // str3 is not accessable here
    //below line will through the error
    // println!("Largest String : {}",str3);
}

fn longest<'a>(x : &'a str,y : &'a str) -> &'a str{
    if x.len() < y.len() {
        y
    }else{
        x
    }
}