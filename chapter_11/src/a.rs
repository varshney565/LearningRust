pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_failed() {
        panic!("Error---------------");
    }

    #[test]
    fn it_works1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        //assert_eq!(a,b) --> it will check whether a is equal to b or not 
        //if equal test passed else test failed
    }

    #[test]
    fn it_works3() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
