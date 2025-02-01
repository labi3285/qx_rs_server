#[allow(unused)]

use rand::{thread_rng, Rng};  
use std::string::String;


pub fn rand_str(len: usize) -> String {  
    let mut rng = thread_rng(); 
    let mut res = String::with_capacity(len);
    for _ in 0..len {  
        let char: u8 = rng.gen_range(33..=126);
        res.push(char as char);
    }  
    res  
}

pub fn rand_digit_str(len: usize) -> String {  
    let mut rng = thread_rng();
    let mut res = String::with_capacity(len);
    for _ in 0..len {  
        let digit: u8 = rng.gen_range(b'0'..=b'9'); 
        res.push(digit as char);
    }  
    res  
} 