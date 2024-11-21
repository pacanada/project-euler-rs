// <p>$2^{15} = 32768$ and the sum of its digits is $3 + 2 + 7 + 6 + 8 = 26$.</p>
// <p>What is the sum of the digits of the number $2^{1000}$?</p>
use num::{BigUint, FromPrimitive};
fn approach_2(exponent: u32) -> u32 {
    let n: u32 = 2_u32.pow(exponent);
    // string approach (will not work for big numbers)
    let string_of_number = n.to_string();
    string_of_number.chars().map(|c| c.to_digit(10).unwrap()).sum()
    

    
}
fn approach_1(exponent: u32) -> u32 {
    // using mod?
    // let mut iter = 2;
    // for i in 2..exponent+1 {
    //     println!("{i}: {}", iter*2);
    //     iter*=2;
    // }
    let n = BigUint::from_u32(2).unwrap().pow(exponent);
    println!("Number: {n}");
    println!("{}", BigUint);
    // string approach (will not work for big numbers)
    let string_of_number = n.to_string();
    string_of_number.chars().map(|c| c.to_digit(10).unwrap()).sum()


    
}

fn main() {

    let exponent = 1000;
    let sol = approach_1(exponent);
    println!("The sum of digits of 2^{exponent} is {sol}");
}
