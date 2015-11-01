use std::ops::{Add, Sub};

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
    type Output = LimitVal;

    fn add(self, other: LimitVal) -> LimitVal {
        let mut result: LimitVal = LimitVal::new().min(0).max(0).val(0).finalize(); //hack
        result.val = self.val + other.val;
        result.max = if(self.max > other.max){
                        other.max // choose the smallest one
                    }else{
                        self.max
                    };
        result.min = if(self.min > other.min){
                        self.min // choose the biggest one
                    }else{
                        other.min
                    };
        result
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
        let x: LimitVal = LimitVal::new().min(10).max(0).val(2).finalize();
        let y: LimitVal = LimitVal::new().min(10).max(0).val(3).finalize();
        
        let z = x + y;
        assert_eq!(z.val,5);
    }
}
