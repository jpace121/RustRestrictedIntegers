use std::ops::{Add, Sub, Mul,Div,Neg,Rem};

pub struct LimitVal {
    val : Result<u8,u8>,
    min : u8, 
    max : u8
}

impl LimitVal {
    
    /// Makes a new value inside the range specified by the macro.
    fn new(val:u8) -> LimitVal {
        if val > 0 && val < 10 { //0 and 10 replaced by macro for realz
            LimitVal{val: Ok(val), min: 0, max: 10} //0 and 10 used for illustration
        } else {
            LimitVal{val: Err(val), min: 0, max: 10} //0 and 10 used for illustration
        }
    }
    
    /// Pulls out the inner value if Ok, else panics.
    fn unwrap(&self) -> u8 {
        self.val.unwrap()
    }
    
    /// Pulls out the inner value if Err, else panics.
    fn unwrap_err(&self) -> u8 {
        self.val.unwrap_err()
    }
    
    /// Pulls out the inner value if ok, else prints msg.
    fn expect(&self, msg: &str) -> u8 {
       self.val.expect(msg)
    }

    /// Runs f on the current value.
    fn map<F>(&self, op:F) -> LimitVal
        where F: Fn(u8) -> u8 {
        match self.val {
            Ok(x) => LimitVal::new(op(x)),
            Err(x) => LimitVal::new(op(x))
        }

    }

    /// Runs f on val if val is err, else returns val.
    fn fix<F>(&self, op:F) -> LimitVal
        where F: Fn(u8) -> u8 {
        match self.val {
            Ok(x) => LimitVal::new(x),
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
    #[test]
    fn test_fix() {
        let x = LimitVal::new(20);
        let f = | t : u8 | {
            let mut z : u8 = t;
            while z > x.max {
                z = t / 10;
            }
            z
        };
        let y = x.fix(f);
        assert_eq!(y.unwrap(),2);
    }
}
