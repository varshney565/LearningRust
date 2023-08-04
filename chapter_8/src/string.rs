use std::fmt::format;
use unicode_segmentation::UnicodeSegmentation;
pub fn stringUse(){
    //declaring strings
    let data = "initial contents";

    let s = data.to_string();

    let s = "initial contents".to_string();

    let mut s = String::from("initial contents");

    //appending
    s.push_str(" -- appending");
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used because ownership has been transfered to s3.
    
    //instead we should do that
    //let s3 = s1.clone() + &s2;

    /*
     * Internally what is happening when we are calling the '+'
     * 
     * it is calling the add function which looks like this.
     * 
     * fn add(self,s:&str) -> String {}
     */

    //another way of appending
    let s4 = format!("{}{}",s3,s2);
    println!("String s4 : {s4}");
    //and it will not take the ownership of s3

    //bytes format
    let s5 = String::from("नमस्ते");
    for b in s5.bytes() {
        print!("{b} ");
    }
    print!("\n");

    //scaler values
    for c in s5.chars() {
        println!("{c}");
    }
    print!("\n");

    //graphic values
    for g in s5.graphemes(true){
        println!("{g}");
    }
    println!();
}