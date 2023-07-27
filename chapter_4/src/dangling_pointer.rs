// pub fn dangle(){
//     let s = dangle();
// } 

//Here we are returning the reference of a variable that is going to be destroyed after that 
//function is done its execution.

// fn dangle() -> &String {
//     let s = String::from("Hello");
//     return &s;
// }