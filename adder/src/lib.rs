#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    #[should_panic(expected = "Make this test fail")]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn rectangle_can_hold_other() {
        let r1 = Rectangle{width: 2, length: 3};
        let r2 = Rectangle{width: 1, length: 2};
        assert!(r1.can_hold(&r2));
    }

    #[test]
    fn rectangle_cannot_hold_other() {
        let r1 = Rectangle{width: 1, length: 2};
        let r2 = Rectangle{width: 1, length: 2};
        assert!(!r1.can_hold(&r2));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn it_does_not_add_three() {
        assert_ne!(6, add_two(3));
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}



#[derive(Debug, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub length: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.length > other.length {
            return true;
        }
        else {
            return false;
        }
    }
}



pub fn add_two(x: i32) -> i32 {
    return x + 2;
}