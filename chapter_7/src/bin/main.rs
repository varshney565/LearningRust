use chapter_7::module1;
mod temp;
use chapter_7::module2;

mod Cards{
    pub fn supply(){
        println!("52");
    }
}
fn main(){
    temp::Motivation();
    module1::Motivation1();
    module2::Motivation2();
    crate::Cards::supply();
}
