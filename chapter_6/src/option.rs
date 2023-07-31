pub fn  option(){
    let t : Option<i8> = Some(1);
    println!("value is {:?}",t);
    let q = 12;
    let r = q+t.unwrap_or(0);
    println!("value of r : {} ",r);
    let s : Option<String> = Some(String::from("Hello"));
    println!("str : {:?}",s);
}