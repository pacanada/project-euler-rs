// <p>The sum of the primes below $10$ is $2 + 3 + 5 + 7 = 17$.</p>
// <p>Find the sum of all the primes below two million.</p>

use project_euler::utils::is_prime;


fn approach_1(n:u64)-> u64 {
    // let v_primes: Vec<u64> = (1..n).filter(|&x| is_prime(x)).collect();
    // println!("{:?}", v_primes);
    (1..n).filter(|&x| is_prime(x)).sum()
    
 
}



fn main() {
    let n=2_000_000;
    let sol = approach_1(n);
    println!("The sum of the primes less than {n} is {sol}");
}
