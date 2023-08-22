use std::mem;

struct Students{
    a : bool,
    b : i32
}

struct T{
    a : String,
    b : i32
}

impl T{
    fn a(&self)->String{
        self.a.clone()
        //we can't write self.a because we can't pass the ownership from a reference.
    }
}

struct shape{

}

fn main() {
    let t = mem::size_of::<Students>();
    println!("{:?}",t);
    let mut v : Vec<i32> = vec![1,2,3,4];
    let k = &v[1];
    v[1] = 23;
    //k is not used here

    let t = T{
        a : "shivam".to_string(),
        b : 12
    };
}