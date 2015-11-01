use std::ops::{Add, Sub};

#[derive(Debug,PartialEq,Eq)]
pub struct LimitVal {
    Val : u8,
    Max : u8,
    Min : u8,
    //Fix : Fn(u8) -> u8 //This caused wierd issues, ignore for now.
}


impl Add for LimitVal {
    type Output = LimitVal;

    fn add(self, other: LimitVal) -> LimitVal {
        let mut result: LimitVal = LimitVal::new().Min(0).Max(0).Val(0).finalize(); //hack
        result.Val = self.Val + other.Val;
        result.Max = if(self.Max > other.Max){
                        other.Max // choose the smallest one
                    }else{
                        self.Max
                    };
        result.Min = if(self.Min > other.Min){
                        self.Min // choose the biggest one
                    }else{
                        other.Min
                    };
        result
    }
}

impl LimitVal {
    fn new() -> LimitVal {
            LimitVal{Val: 0, Max: 0, Min: 0}
        }
    fn Max(&mut self , val:u8) -> &mut LimitVal{
        self.Max  = val;
        self
    }
    fn Min(&mut self , val:u8) -> &mut LimitVal{
        self.Min  = val;
        self
    }
    fn Val(&mut self , val:u8) -> &mut LimitVal{
        self.Val  = val;
        self
    }
    fn finalize(&self) -> LimitVal{
        LimitVal{Max: self.Max, Min: self.Min, Val: self.Val}
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
        let x: LimitVal = LimitVal::new().Min(10).Max(0).Val(2).finalize();
        let y: LimitVal = LimitVal::new().Min(10).Max(0).Val(3).finalize();
        
        let z = x + y;
        assert_eq!(z.Val,5);
    }
}
