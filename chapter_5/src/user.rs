use crate::user;

#[derive(Debug)]

struct User{
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool
}

fn create_user(username : String,email : String,active : bool,sign_in_count : u64) -> User{
    User {
        username,
        email,
        sign_in_count,
        active
    }
}

pub fn usecase() {
    let mut user1 = User{
        email : String::from("shivamvarshney565@gmail.com"),
        username : String::from("shivam565"),
        active : true,
        sign_in_count : 1
    };

    println!("obj : {:?}",user1); 

    let mut user2 = create_user(
        user1.username.clone(),
        String::from("email"),
        true,
        1
    );
}


//Tuple struct

fn demo(){
    struct Point(i32,i32);
    let p1 = Point(1,2);
    let p2 = Point(-1,-2);
    let p3 = (1,3);
}