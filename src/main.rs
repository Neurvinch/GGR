use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

 impl Guess {

     pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!(
                "The secret number will be between 1 and 100, got {}."
            );
         }
         Guess {value}
        }
    
        
        
 }



