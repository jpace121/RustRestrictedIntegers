use std::ops::{Add, Sub};

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub enum LimitVal {
    Val(u8)
}


impl Add for LimitVal {
    type Output = LimitVal;

    fn add(self, other: LimitVal) -> LimitVal {
        let val1 = match self {
            LimitVal::Val(x) => x,
        };

        let val2 = match other {
            LimitVal::Val(x) => x,
        };

        LimitVal::Val(val1+val2)

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
        let x: LimitVal = LimitVal::Val(1);
        let y = LimitVal::Val(2);
        let z = x + y;
        assert_eq!(z,LimitVal::Val(3));
    }
}
