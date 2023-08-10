
struct Rectangle{
    width : i32,
    height : i32
}

impl Rectangle{
    fn can_hold(&self,other : &Rectangle) -> bool{
        if self.width >= other.width && self.height >= other.height {
            return true;
        }
        return false;
    }
}

#[derive(Debug,PartialEq)]
struct Point{
    x : i32,
    y : i32
}


pub fn add_two(val : i32) -> i32 {
    val+2
}

#[cfg(test)]
mod tests{

    /**
     * For assert_eq!() and assert_ne!() we have to implement our own 
     * Debug and PartialEq traits
     */

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width : 8,
            height : 7
        };
        let smaller = Rectangle {
            width : 5,
            height : 1
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_not_hold_larger() {
        let larger = Rectangle {
            width : 8,
            height : 7
        };
        let smaller = Rectangle {
            width : 5,
            height : 1
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_add_two() {
        assert_ne!(4,add_two(1));
    }


    //testing points

    #[test]
    fn test_points(){
        let p1 = Point{x : 1,y : 2};
        let p2 = Point{x : 2,y : 3};
        assert_ne!(p1,p2);
    }


    //---->error message
    // assert!(condition,custome_error_msg)

    #[test]
    #[should_panic(expected="ERROR ERROR ALERT ERROR ERROR !!")]
    fn custom_error(){
        assert!(false,"ERROR ERROR ALERT ERROR ERROR !!");
    }

    #[test]
    fn test_result_return_error()-> Result<(),String> {
        if(2 + 2 == 4) {
            Ok(())
        }else{
            Err(String::from("Error !!"))
        }
    }
}