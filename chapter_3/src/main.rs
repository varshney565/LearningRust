fn main() {
    /*
     * part 1:
     * shadowing and how to declare variables in rust
     */

    let x = 12;
    //for making a variable mutable we should use "mut"
    const T : u128 = 12;
    print!("value of x is {x} and value of T is {T}\n");

    //shadowing
    let x = "shivam";
    println!("{x}");

    /*
     * part 2 : 
     * rust has two types of data types
     * 
     * a) scaler data types
     * 
     * 
     * 1) Integers
     * 2) Floating-point numbers
     * 3) character
     * 
     * 
     * b) compound data types
     */

//scaler data types

//integers
    let a : i8 = 12;      //decimal
    let b : i8 = 0x1f;   //hexadecimal
    let c : i8 = 0b0101; //binary
    let d : u8 = b'A';   //byte
    println!("byte : {d}");

    //isize and usize depends on the system architecture
    //isize --> signed
    //usize --> unsigned

    // for calculating the size
    let a4 : isize = isize::MAX;
    println!("value of the size is {a4}");

    let e : u8 = 255;
    let exp = -5%3;
    println!("{exp}");
//boolean

    let a1 : bool = true;
    let b1 : bool = false;
//character

    let a2 : char = 'z';
//floating point numbers

    let k = -8.0; //by default f64
    let p = k/2.1;
    println!("{p}");




//compound data type

//tuples
    let a3 : (&str,i8) = ("Hello",2);
    let (a5,b5) = a3;
    //or
    let a6 = a3.1;
    let b6 = a3.0;
    println!("value of the tuple is : {b6},{a6}");

//arrays
    let p = [1,2,3,4];
    let l = p.len();
    println!("len : {l}");
    //for accessing the elements simply use []
     
    //create an array with all the values set to a default value
    let arr = [0;10];

//calling functions
    let op = go(1,2);
    println!("1 + 2 = {op}");

//control flow

//If else
    let age = 12;
    if age > 18 {
        println!("Yes you can drink alcohol !");
    }else{
        println!("Kids don't do such stuffs !");
    }

    let condition = true;
    let number = if condition {let t = 12;let p = 13;t+p+5} else {6};
    println!("number is : {number}");

//Loops

    //using "loop"
    let mut i = 0;
    let mut sum = 0;
    loop {
        if i == 10 {
            break;
        }
        sum += i;
        i += 1;
    }
    println!("sum : {sum}");


    //using "while"
    i = 0;
    sum = 0;
    while i < 10 {
        sum = sum + i;
        i = i+1;
    }
    println!("Again the sum is : {sum}");

    //using "for"
    let arr = [1,2,3,4,5];
    let mut its = arr.iter();
    let firstEle = its.next();
    //iterator is the pointer that is pointing to the current element of the array

    //it will print elements from 2 to 5
    for element in its{
        println!("the value is : {}",element);
    }

    //using "for" for range
    for number in 1..5 {
        println!("value is : {}",number);
    }
}


// functions
fn go(x : i64,y : i64) -> i64{
    return x+y;
}
