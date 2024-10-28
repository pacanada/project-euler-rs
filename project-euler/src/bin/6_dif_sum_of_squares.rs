
// <p>The sum of the squares of the first ten natural numbers is,</p>
// $$1^2 + 2^2 + ... + 10^2 = 385.$$
// <p>The square of the sum of the first ten natural numbers is,</p>
// $$(1 + 2 + ... + 10)^2 = 55^2 = 3025.$$
// <p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
// <p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>
fn approach_1(n: u32) -> u32 {
    // square of sum of n natural numbers: (1+2+..+n)^2 = ((n+1)*n/2)^2
    let square_of_sum = (n*(n+1)/2).pow(2);
    dbg!(square_of_sum);
    let sum_of_squares: u32 = (1..n+1).map(|x| x.pow(2)).sum();
    dbg!(sum_of_squares);

    square_of_sum-sum_of_squares
    }

fn main() {
    let n: u32 = 100;
    let solution = approach_1(n)

;
    println!(
        "Difference for {}: {:?}", n, solution
    )
    
}
