// <p>By listing the first six prime numbers: $2, 3, 5, 7, 11$, and $13$, we can see that the $6$th prime is $13$.</p>
// <p>What is the $10\,001$st prime number?</p>

use project_euler::utils;

fn approach_1(n: u32) -> u32 {
    let mut number = 0;
    let mut i = 1;
    loop {
        if utils::is_prime(number) {
            i+=1;
            if i==n+1 {break}

        }
        number+=1;

    }
    number
    
    }

fn main() {
    let n: u32 = 10001;
    let solution = approach_1(n)

;
    println!(
        " {}th prime number: {:?}", n, solution
    )
    
}
