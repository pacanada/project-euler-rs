// <p>The prime factors of $13195$ are $5, 7, 13$ and $29$.</p>
// <p>What is the largest prime factor of the number $600851475143$?</p>
fn is_prime(n: u64)-> bool {
    let mut is_prime = true;
    if (n == 1) | (n==2) {
        return true
    }
    for i in 2..n/2 {
        if n%i==0 {
            is_prime = false;
            break
        }
    }
    is_prime
}
fn approach_1(n: u64) -> u64 {
    let mut quotient = n;
    let mut divisor = 2;
    let mut factors = Vec::<u64>::new();
    loop {
        if quotient==1 {
            break
        }
        if (is_prime(divisor)) & (quotient%divisor==0) {
            quotient/=divisor;
            factors.push(divisor);
            //println!("quotient {} Divisor {}", quotient, divisor);
        }
        divisor+=1;
    }

println!("{:?}", factors);
*factors.iter().max().unwrap()
}

fn main() {
    let n: u64 = 600851475143;
    let solution = approach_1(n)

;
    println!(
        "Largest prime factor of  {}: {}", n, solution
    )
    
}
