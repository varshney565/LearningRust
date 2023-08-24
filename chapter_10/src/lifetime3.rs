struct ImportantInfo<'a>{
    part : &'a str
}

impl<'a> ImportantInfo<'a> {
    fn return_part(&self,announcement : &str) -> &str {
        println!("Attention Please : {}",announcement);
        self.part
    }
}

pub fn usecase() {
    let f;
    {
        let info = "Shivam Varshney".to_string();
        f = ImportantInfo {
            part : info.as_str()
        };
        let k : &'static str = "shivam";
    }
    //we can't use 'f' here
    
}
