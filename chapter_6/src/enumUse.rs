#[derive(Debug)]
enum InAddKind{
    V4,V6
}

#[derive(Debug)]
struct IpAdd {
    kind : InAddKind,
    address : String
}

pub fn IP() {
    let four : InAddKind = InAddKind::V4;
    let six : InAddKind = InAddKind::V6;
    println!("Value is : {:?} and {:?}",four,six);

    let ip1 : IpAdd = IpAdd {
        kind : InAddKind::V4,
        address : String :: from("127.0.0.1")
    };
    println!("Address is : {:?}",ip1);
}

//another way of doint the same thing

#[derive(Debug)]
enum Ip {
    V4(String),
    V6(String)
}

impl Ip{
    fn print(&self){
        println!("IP address : {:?}",self);
    }
}

pub fn Ip(){
    let ip1 = Ip::V6(String :: from("127.0.0.1"));
    let ip2 = Ip::V4(String :: from("127.80.12.1"));
    ip1.print();
    ip2.print();
}