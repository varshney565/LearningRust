use std::fs::{self,File};
use std::io::ErrorKind;
use std::io::Read;
use std::io;

pub fn error_propogation() -> Result<String,io::Error>{
    //? --> it will return the content of the file in case of no error
    //      else return Error in case of error
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    //or
    // File::open("hello.txt")?.read_to_string(&mut s)
    //or
    // fs::read_to_string("hello.txt");
}

pub fn use_it(){
    // enum Result<T,E>{
    //     Ok(T),
    //     Err(E)
    // }

    let file_result = File::open("Hello.txt");
    let _file = match file_result{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problme creating the file : {:?}",e)
            },
            error => panic!("error : {} while opening the file",error)
        }
    };

    //what else we can do
    let _file = File::open("hello.txt").unwrap();
    //---> it file exists than it will return the file else it will automatically through the error

    //also it will also work
    println!("Yes !");
    let _file = File::open("hello.txt").expect("hello.txt ---> such file does't exist !");
    //---> returns file if exist and through the error message that is in the expect in case of error.

}