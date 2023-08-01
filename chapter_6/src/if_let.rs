pub fn last(){
    let maximum = Some(3);
    if let Some(max) = maximum {
        println!("Value is : {}",max);
    }else{
        println!("None");
    }

    let t = String::from("Shivam");
    if let "shivam" = t.as_str() {
        println!("Yes");
    }else{
        println!("No");
    }
}