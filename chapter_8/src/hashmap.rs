use std::collections::HashMap;

pub fn hashmapUse(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 1);
    scores.insert(String::from("Yellow"), 50);

    let V = scores.get("Blue");
    match V {
        Some(val) => println!("color's val at Location : {:p}",val),
        _ => println!("No such color !")
    }
    scores.insert(String::from("Blue"), 4);
    let k = scores.get("Blue");
    match k {
        Some(val) => println!("color's val at Location : {:p}",val),
        _ => println!("No such color !")
    }

    //Option<> will always take the padding
    //because None will always take the padding
}