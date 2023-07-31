#[derive(Debug)]
enum Other{
    CEO(i32),
    CTO(i32)
}

enum Name {
    Shivam,
    Hritik,
    Vishal,
    Lovish,
    Tiwari(Other)
}

pub fn patmat(){
    //matching in enum
    let name = Name :: Tiwari(Other :: CTO(12));

    match name {
        Name :: Shivam => {
            println!("blockchain !");
        },
        Name :: Vishal => {
            println!("java developer !");
        },
        Name :: Hritik => {
            println!("backend developer !");
        },
        Name :: Lovish => {
            println!("Machine Learning !");
        },
        Name :: Tiwari(Other :: CEO(salary)) => {
            println!("CEO : {}",salary);
        },
        Name :: Tiwari(Other :: CTO(salary)) => {
            println!("CTO : {}",salary);
        }
    }
    //matching in String    
    let s = String::from("yes");
    match s.as_str(){
        "yes" => {
            println!("1");
        },
        _ => {
            println!("0");
        }
    }

    //matching in option<i32>
    let a : Option<f32> = Some(1.3);
    let b : Option<f32> = Some(1.2);
    let t = doDivision(&a,&b);
    println!("Division of a : {:?} and b : {:?} = {:?}",a,b,t);
} 

fn doDivision(a : &Option<f32>,b : &Option<f32>) -> Option<f32> {
    match b {
        Some(0.0) => None,
        Some(val1) => {
            match a{
                Some(val2) => Some(val2/val1),
                None => None
            }
        },
        None => None
    }
}



enum Month {
    Jan,Feb,Mar
}
