//
//<p>If we list all the natural numbers below $10$ that are multiples of $3$ or $5$, we get $3, 5, 6$ and $9$. The sum of these multiples is $23$.</p>
//<p>Find the sum of all the multiples of $3$ or $5$ below $1000$.</p>
//
fn approach_1(n: u32) -> u32 {
    let mut multiples_of_3_or_5 = Vec::<u32>::new();
    for i in 1..n {
        if (i % 3 == 0) | (i % 5 == 0) {
            multiples_of_3_or_5.push(i);
        }
    }
    multiples_of_3_or_5.iter().sum::<u32>()

}
fn approach_2(n: u32) -> u32 {
    (1..n).filter(|i| (i%3==0) | (i%5==0)).sum()
}
fn main() {
    let n: u32 = 1000;
    // using for loops
    //let sum = approach_1(n);
    // using iter and filter
    let sum: u32 = approach_2(n);
    println!(
        "The sum of multiples of 3 or 5 between 0 and {} is {}",
        n,
        sum
    )
    
}
