trait PrintTwice {
    fn print_twice(&self);
}

impl<T: ToString> PrintTwice for T {
    fn print_twice(&self) {
        let string = self.to_string();
        println!("{} {}", string, string);
    }
}

pub fn use_me() {
    let number = 42;
    let text = "Hello, world!";
    
    number.print_twice(); // Prints: 42 42
    text.print_twice();   // Prints: Hello, world! Hello, world!
}
