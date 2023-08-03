fn tell(){
    println!("Yes !!");
}

mod calculator{
    pub fn add(a:i32,b:i32)->i32{
        super::tell(); //one directory above
        a+b
    }
    pub fn multi(a:i32,b:i32)->i32{
        a*b
    }
}

mod enum_struct{
    #[derive(Debug)]
    pub struct person{
        pub name : String,
        gender : String
    }

    impl person {
        pub fn createMale(name : &str) -> person{
            person { 
                name: (String::from(name)), 
                gender: (String::from("M")) 
            }
        }

        pub fn createFemale(name : &str) -> person{
            person { 
                name: (String::from(name)), 
                gender: (String::from("F")) 
            }
        }
    }
}

pub fn calculatorUse(){
    //absulute path
    let add : i32 = crate::calculator::add(1, 2);
    //relative path
    let mul : i32 = self::calculator::multi(1, 2);
    println!("addition : {}, multiplication : {}",add,mul);
}

pub fn enum_structUse(){
    let mut p1 = enum_struct::person::createMale("shivam");
    println!("Person : {:?}",p1);
    p1.name = String::from("Varshney");
    println!("Person : {:?}",p1);
}