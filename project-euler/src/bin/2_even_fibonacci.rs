// <p>Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with $1$ and $2$, the first $10$ terms will be:
// $$1, 2, 3, 5, 8, 13, 21, 34, 55, 89, \dots$$</p>
// <p>By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.</p>
//
fn approach_1(n: u32) -> u32 {

    let mut fib_numbers = vec![1,2];
    loop {
        let first = fib_numbers[fib_numbers.len()-2];
        let second = fib_numbers[fib_numbers.len()-1];
        if first+second >= n {
            break
        }
        
        fib_numbers.push(first+second);

    }
    //println!("{:?}", fib_numbers);
    fib_numbers.iter().filter(|&x| x%2==0).sum()

}

fn main() {
    let n: u32 = 4_000_000;
    let solution = approach_1(n)

;
    println!(
        "Sum of even fibonacci number smaller than {}: {}", n, solution
    )
    
}
