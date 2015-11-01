use std::ops::{Add, Sub, Mul};

pub struct LimitVal {
    val : u8,
    max : u8,
    min : u8,
    //Fix : Fn(u8) -> u8 //This causes issue with size not being implemented?
}

impl LimitVal {
    fn new() -> LimitVal {
            LimitVal{val: 0, max: 0, min: 0}
        }
    fn max(&mut self , val:u8) -> &mut LimitVal{
        self.max  = val;
        self
    }
    fn min(&mut self , val:u8) -> &mut LimitVal{
        self.min  = val;
        self
    }
    fn val(&mut self , val:u8) -> &mut LimitVal{
        self.val  = val;
        self
    }
    fn finalize(&self) -> LimitVal{
        LimitVal{max: self.max, min: self.min, val: self.val}
    }

}

impl Add for LimitVal {
    type Output = Option<LimitVal>;

    fn add(self, other: LimitVal) -> Option<LimitVal> {
        let mut result: LimitVal = LimitVal::new()
                                  .min(self.min)
                                  .max(self.max)
                                  .val(0)
                                  .finalize(); //hack?
        result.val = self.val + other.val;
        if result.val > result.max {
            None
        } else if result.val < result.min {
            None
        }else{
            Some(result)
        }
    }
}

impl Sub for LimitVal {
    type Output = Option<LimitVal>;

    fn sub(self, other: LimitVal) -> Option<LimitVal> {
        let mut result: LimitVal = LimitVal::new()
                                  .min(self.min)
                                  .max(self.max)
                                  .val(0)
                                  .finalize(); //hack?
        result.val = self.val - other.val;
        if result.val > result.max {
            None
        } else if result.val < result.min {
            None
        }else{
            Some(result)
        }
    }
}

impl Mul for LimitVal {
    type Output = Option<LimitVal>;

    fn mul(self, other: LimitVal) -> Option<LimitVal> {
        let mut result: LimitVal = LimitVal::new()
                                  .min(self.min)
                                  .max(self.max)
                                  .val(0)
                                  .finalize(); //hack?
        result.val = self.val * other.val;
        if result.val > result.max {
            None
        } else if result.val < result.min {
            None
        }else{
            Some(result)
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
        let x: LimitVal = LimitVal::new().max(10).min(0).val(2).finalize();
        let y: LimitVal = LimitVal::new().max(10).min(0).val(3).finalize();
        
        let z = x + y;
        assert_eq!(z.unwrap().val,5);
    }

    #[test]
    fn test_sub() {
        let x: LimitVal = LimitVal::new().max(10).min(0).val(2).finalize();
        let y: LimitVal = LimitVal::new().max(10).min(0).val(3).finalize();
        
        let z = y - x;
        assert_eq!(z.unwrap().val,1);
    }
    #[test]
    fn test_mul() {
        let x: LimitVal = LimitVal::new().max(10).min(0).val(2).finalize();
        let y: LimitVal = LimitVal::new().max(10).min(0).val(3).finalize();
        
        let z = y * x;
        assert_eq!(z.unwrap().val,6);
    }
}
