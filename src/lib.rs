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
        let mut result: LimitVal = LimitVal{Val: 0, Max: 0, Min: 0}; //hack
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
        let mut x: LimitVal = LimitVal{Val: 2, Max: 10, Min: 0};
        let mut y: LimitVal = LimitVal{Val: 3, Max: 10, Min: 0};
        
        let z = x + y;
        assert_eq!(z.Val,5);
    }
}
