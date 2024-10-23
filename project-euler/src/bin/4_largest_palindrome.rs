// <p>A palindromic number reads the same both ways. The largest palindrome made from the product of two $2$-digit numbers is $9009 = 91 \times 99$.</p>
// <p>Find the largest palindrome made from the product of two $3$-digit numbers.</p>
fn approach_1(n: u32) -> u32 {
    // brute force
    let max_number = 10_u32.pow(n)-1;
    let min_number = 10_u32.pow(n-1)-1;
    let mut palindromes = Vec::<u32>::new();
    println!("Max: {} Min: {}", max_number, min_number);
    for i in min_number..max_number {
        for j in min_number..max_number {
            let multiplication = i*j;
            let multiplication_string = multiplication.to_string();
            if multiplication_string == multiplication_string.chars().rev().collect::<String>() {
                palindromes.push(multiplication);
            }
            

        }
    }
    *palindromes.iter().max().unwrap()
 
}

fn main() {
    let n: u32 = 3;
    let solution = approach_1(n)

;
    println!(
        "Largest palindrom product of two numbers of {} digits: {}", n, solution
    )
    
}
