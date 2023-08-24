/*
 *Rules :  
 * 
 * 
 * The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a
 * reference. In other words, a function with one parameter gets one lifetime parameter: 
 * fn foo<'a>(x: &'a i32);
 * fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
 * 
 *  
 * The second rule is that, if there is exactly one input lifetime parameter, that lifetime is 
 * assigned to all output lifetime parameters
 * fn foo<'a>(x: &'a i32) -> &'a i32
 * 
 * 
 * The third rule is that, if there are multiple input lifetime parameters, but one of them is 
 * &self or &mut self because this is a method, the lifetime of self is assigned to all output
 * lifetime parameters.
 * fn first_word<'a>(s: &'a str) -> &'a str
 * 
 */

 /*
  *Static lifetime
  */

  //as we know all the string literals stored in the binary of the file
  //and they live as long as program is running mean they remains in the static memory
  

  //let s : &'static str = "shivam"; --> static lifetime
  //--->by default it has static lifetime but we are just adding more clearity here.

  use std::fmt::Display;

  fn longest_with_announcement<'a,T>(x :&'a str,y : &'a str,ann : T) -> &'a str
  where T : Display
  {
    println!("Announcement! {}",ann);
    if x.len() > y.len() {
        x
    }else {
        y
    }
  }

  pub fn use_me() {
    
  }