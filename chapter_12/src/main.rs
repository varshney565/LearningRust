use std :: env;
use std :: fs;
use std :: process;
use std :: error :: Error;
struct Config {
    query : String,
    filename : String
}

impl Config {
    fn new(args : &[String]) -> Result<Config,&str>{
        if args.len() < 3 {
            return Err("---->Not enough arguments !!<----");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{query,filename})
    }
}

fn Logic(conf : &Config) {
    let content = fs::read_to_string(conf.filename.as_str()).expect("Error reading file !!");
    println!("{}",content);
}
fn main () {
    let args : Vec<String> = env :: args().collect();
    let res = Config::new(&args);
    match res {
        Ok(conf) => {
            Logic(&conf);
        },
        Err(str) => {
            println!("\n{str}\n");
            process::exit(1);
        }
    }
}