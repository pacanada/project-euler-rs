use std::collections::HashMap;

use num::integer::Roots;
use project_euler::utils;

fn approach_1(n: u32) -> u32 {
    // brute force
    let mut m:HashMap<u32, u32>  = HashMap::new();
    for i in 1..=n {
        let divisors: Vec<u32> = (1..=i/2+1).filter(|x| i%x == 0).collect();
        m.insert(i, divisors.iter().sum());
        println!("{i}: {:?}", divisors);
    }
    println!("{:?}", m);
    
    0
    }
fn approach_2(n: u32) -> u32 {
    fn sum_of_divisors(n: u32)->u32{
        let prime_factors = utils::get_factors(n).unwrap();
        let mut divisors = vec![1];
        // iterate through all possible combinations of factors, using bits 2**2 for all possible combinations of factors
        // 0000, 0001, 0010, ..., [f1*0*f2*0*f3*0*f4*0]. It probably has more algorithm complexity than just brute force
        0
    }
    
    sum_of_divisors(n)
    }
    
    fn main() {
    
        let n = 12;
        let sol = approach_1(n);
        println!(" {n}  is {sol}");
    }