use chapter_12::*;
use std :: process;
use std :: env;

fn main () {
    let args : Vec<String> = env :: args().collect();
    let res = Config::new(&args);
    match res {
        Ok(conf) => {
            let res = logic(&conf);
            if let Err(e) = res {
                println!("{e}\n");
                process::exit(1);
            }
        },
        Err(str) => {
            println!("\n{str}\n");
            process::exit(1);
        }
    }
}