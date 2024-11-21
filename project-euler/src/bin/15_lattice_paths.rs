// <p>Starting in the top left corner of a $2 \times 2$ grid, and only being able to move to the right and down, there are exactly $6$ routes to the bottom right corner.</p>
// <div class="center">
// <img src="resources/images/0015.png?1678992052" class="dark_img" alt=""></div>
// <p>How many such routes are there through a $20 \times 20$ grid?</p>
fn approach_1(size: u32) -> u128 {
    fn factorial(n: u128)->u128{
        (2..n+1).fold(1, |acc, x| acc*x)

    }
    fn factorial_over_factorial(numerator: u128, denominator: u128 )->u128 {
        (denominator+1..numerator+1).fold(1, |acc, x| acc*x)

    }
    // with pure math formula for the 2x2 grid: R: right, D: down we need to compute the possible 
    //combinations in the set of 4(n) steps with 2(k_i) directions of each type (repeated values):
    // DDRR, DRDR, DRRD, RRDD, RDRD, RDDR
    // This is same case as the possible arrangements of the word Mississippi (well known example):
    // n! / (k_1!k_2!)
    // we need to be careful with the precission of the integer, a naive factorial division will not work because of the huge numbers produced by the factorial
    let n = size*2;
    factorial_over_factorial(n as u128, size as u128) / factorial(size as u128)
    

    
}

fn main() {

    let size = 20;
    let sol = approach_1(size);
    println!("The of possible combinations for {size} x {size} grid is: {sol}");
}
