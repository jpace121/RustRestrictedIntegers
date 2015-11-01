use std::ops::{Add, Sub, Mul};

pub struct LimitVal {
    val : Option<u8>,
    min : u8, 
    max : u8
}

impl LimitVal {
    fn new(val:u8) -> LimitVal {
        if(val > 0 && val < 10) { //0 and 10 replaced by macro for realz
            LimitVal{val: Some(val), min: 0, max: 10} //0 and 10 used for illustration
        } else {
            LimitVal{val: None, min: 0, max: 10} //0 and 10 used for illustration
        }
    }
    fn new_none() -> LimitVal {
        //returns val as None, should be private, only for inside the library.
        //I think that there has to be a better way to do this.
            LimitVal{val: None, min: 0, max: 10} //0 and 10 used for illustration
    }
}

impl Add for LimitVal {
    type Output = LimitVal;

    fn add(self, other: LimitVal) -> LimitVal {
        match self.val {
            Some(x) => {
                match other.val {
                    Some(y) => LimitVal::new(x+y), //both are Some
                    None => LimitVal::new_none() //other.val is None
                }
            }
            None => LimitVal::new_none() //self.val is None
        }
    }
}

impl Sub for LimitVal {
    type Output = LimitVal;

    fn sub(self, other: LimitVal) -> LimitVal {
        match self.val {
            Some(x) => {
                match other.val {
                    Some(y) => LimitVal::new(x-y), //both are Some
                    None => LimitVal::new_none() //other.val is None
                }
            }
            None => LimitVal::new_none() //self.val is None
        }
    }
}

impl Mul for LimitVal {
    type Output = LimitVal;

    fn mul(self, other: LimitVal) -> LimitVal {
        match self.val {
            Some(x) => {
                match other.val {
                    Some(y) => LimitVal::new(x*y), //both are Some
                    None => LimitVal::new_none() //other.val is None
                }
            }
            None => LimitVal::new_none() //self.val is None
        }
    }
}

#[cfg(test)]
mod tests {
    /*
    Can always put in own file, then would need:
    extern crate adder; (or whatever)
    and then namespace:: the functions.
    */
    use super::*;
        
    #[test]
    fn test_add() {
        let x: LimitVal = LimitVal::new(2);
        let y: LimitVal = LimitVal::new(3);
        
        let z = x + y;
        assert_eq!(z.val.unwrap(),5);
    }

    #[test]
    fn test_sub() {
        let x: LimitVal = LimitVal::new(2);
        let y: LimitVal = LimitVal::new(3);
        
        let z = y - x;
        assert_eq!(z.val.unwrap(),1);
    }
    #[test]
    fn test_mul() {
        let x: LimitVal = LimitVal::new(2);
        let y: LimitVal = LimitVal::new(3);
        
        let z = y * x;
        assert_eq!(z.val.unwrap(),6);
    }
}
