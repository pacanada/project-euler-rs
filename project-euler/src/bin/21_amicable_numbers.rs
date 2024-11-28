use std::collections::HashMap;

use project_euler::utils;

fn approach_1(n: u32) -> u32 {
    // brute force
    let mut v: Vec<u32> = Vec::with_capacity(n as usize);
    // to start with the right index, arrays cannot be used bc n needs to be known at compiled time
    v.push(0);
    for i in 1..=n {
        let divisors: Vec<u32> = (1..=i/2+1).filter(|x| i%x == 0).collect();
        v.push(divisors.iter().sum());
       // println!("{i}: {:?}, sum: {}", divisors, divisors.iter().sum::<u32>());
    }
    let mut amicable_numbers: Vec<u32> = Vec::new();
    for (index, value) in v.iter().enumerate() {
        if *value as usize > v.len() {continue;}
        if (v[*value as usize]==index as u32) & (*value!=index as u32){
            amicable_numbers.push(*value);

        }

    }
    //println!("{:?}", v);
    println!("{:?}", amicable_numbers);
    
    amicable_numbers.iter().sum()
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
    
        let n = 10000;
        let sol = approach_1(n);
        // or even better: https://mathschallenge.net/index.php?section=faq&ref=number/sum_of_divisors
        println!("Sum of all amicable numbers under {n}  is {sol}");
    }