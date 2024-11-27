use num::{BigUint, FromPrimitive};
use project_euler::utils;

fn approach_1(n: BigUint) -> u32 {
    let string = utils::factorial(n).to_string();
    string.chars().fold(0,|acc,x|acc+x.to_digit(10).unwrap())
    }
    
    fn main() {
        let n_prim = 100;
    
        let n = BigUint::from_u32(n_prim).unwrap();
        let sol = approach_1(n);
        println!("Sum of digits of factorial {n_prim}  is {sol}");
    }