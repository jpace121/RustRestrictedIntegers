use std::ops::{Add, Sub, Mul,Div,Neg,Rem};

pub struct LimitVal {
    val : Result<u8,u8>,
    min : u8, 
    max : u8
}

impl LimitVal {
    fn new(val:u8) -> LimitVal {
        if val > 0 && val < 10 { //0 and 10 replaced by macro for realz
            LimitVal{val: Ok(val), min: 0, max: 10} //0 and 10 used for illustration
        } else {
            LimitVal{val: Err(val), min: 0, max: 10} //0 and 10 used for illustration
        }
    }

    fn unwrap(&self) -> u8 {
        self.val.unwrap()
    }

    fn unwrap_err(&self) -> u8 {
        self.val.unwrap_err()
    }

    fn expect(&self, msg: &str) -> u8 {
       self.val.expect(msg)
    }

    fn map<F>(&self, op:F) -> LimitVal
        where F: Fn(u8) -> u8 {
        match self.val {
            Ok(x) => LimitVal::new(op(x)),
            Err(x) => LimitVal::new(op(x))
        }

    }
    
}

impl Add for LimitVal {
    type Output = LimitVal;

    fn add(self, other: LimitVal) -> LimitVal {
        match self.val {
            Ok(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x+y), //both are Ok
                    Err(y) => LimitVal::new(x+y)
                }
            }
            Err(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x+y),
                    Err(y) => LimitVal::new(x+y)
                }
            }
          
        }
    }
}

impl Sub for LimitVal {
    type Output = LimitVal;

    fn sub(self, other: LimitVal) -> LimitVal {
        match self.val {
            Ok(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x-y), //both are Ok
                    Err(y) => LimitVal::new(x-y)
                }
            }
            Err(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x-y),
                    Err(y) => LimitVal::new(x-y)
                }
            }
          
        }
    }
}

impl Div for LimitVal {
    type Output = LimitVal;

    fn div(self, other: LimitVal) -> LimitVal {
        match self.val {
            Ok(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x/y), //both are Ok
                    Err(y) => LimitVal::new(x/y)
                }
            }
            Err(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x/y),
                    Err(y) => LimitVal::new(x/y)
                }
            }
          
        }
    }
}

impl Mul for LimitVal {
    type Output = LimitVal;

    fn mul(self, other: LimitVal) -> LimitVal {
        match self.val {
            Ok(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x*y), //both are Ok
                    Err(y) => LimitVal::new(x*y)
                }
            }
            Err(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x*y),
                    Err(y) => LimitVal::new(x*y)
                }
            }
          
        }
    }
}

impl Rem for LimitVal {
    type Output = LimitVal;

    fn rem(self, other: LimitVal) -> LimitVal {
        match self.val {
            Ok(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x%y), //both are Ok
                    Err(y) => LimitVal::new(x%y)
                }
            }
            Err(x) => {
                match other.val {
                    Ok(y) => LimitVal::new(x%y),
                    Err(y) => LimitVal::new(x%y)
                }
            }
          
        }
    }
}

impl Neg for LimitVal {
    type Output = LimitVal;

    fn neg(self) -> LimitVal {
        match self.val {
            Ok(x) => LimitVal::new(-x),
            Err(y) => LimitVal::new(-y)
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
        assert_eq!(z.unwrap(),5);
    }

    #[test]
    fn test_sub() {
        let x: LimitVal = LimitVal::new(2);
        let y: LimitVal = LimitVal::new(3);
        
        let z = y - x;
        assert_eq!(z.unwrap(),1);
    }
    #[test]
    fn test_div() {
        let x: LimitVal = LimitVal::new(4);
        let y: LimitVal = LimitVal::new(2);

        let z = x/y;
        assert_eq!(z.unwrap(),2);
    }
    #[test]
    fn test_mul() {
        let x: LimitVal = LimitVal::new(2);
        let y: LimitVal = LimitVal::new(3);
        
        let z = y * x;
        assert_eq!(z.unwrap(),6);
    }
    #[test]
    fn test_rem() {
        let x: LimitVal = LimitVal::new(5);
        let y: LimitVal = LimitVal::new(2);
        
        let z = x % y;
        assert_eq!(z.unwrap(),1);
    }
    #[test]
    #[should_panic]
    fn test_neg() {
        let x: LimitVal = LimitVal::new(2);
        let z = -x;
        assert_eq!(z.unwrap(), -2);
    }
    #[test]
    fn test_map() {
        let x: LimitVal = LimitVal::new(2);
        let f = |t| { t + 1};
        
        let z = x.map(f);
        assert_eq!(z.unwrap(),3);
    }
}
