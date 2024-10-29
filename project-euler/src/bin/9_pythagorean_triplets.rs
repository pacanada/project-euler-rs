// <p>A Pythagorean triplet is a set of three natural numbers, $a \lt b \lt c$, for which,
// $$a^2 + b^2 = c^2.$$</p>
// <p>For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.</p>
// <p>There exists exactly one Pythagorean triplet for which $a + b + c = 1000$.<br>Find the product $abc$.</p>

use num::pow;


fn approach_1(n: u32) -> u32 {
    // a^2+b^2 = c^2
    // a+b+c = 1000
    // a<b<c
    // Doing some simplification 
    //      a^2+b^2 = (1000-a-b)(1000-a-b)
    //      a^2+b^2 = 1000000-1000*a-1000*b-1000*a+a^2+a*b-1000*b+a*b+b^2
    //      0       = 1000000-2000*a-2000*b+2*a*b
    // Simplifying for a
    //      2000a+2000b+2ab = 1000000 
    //      a(2000+2b) = 1000000-2000b
    //      a = (1000000-2000b)/(2000+2b) = 2000(500-b)/(2000+2b)

    let a: f32= 10.0;
    let b: f32 = 2000.0*(500.0-a)/(2000.0+2.0*a);
    let c: f32 = (a*a +b*b).sqrt();
    println!("{} {} {}", a,b,c);
    println!("{}", a+b+c);

0
   
}



fn main() {

    let n = 1000;
    let solution = approach_1(n);
    println!(" Product of {} adjacent digits: {:?}", n, solution)
}
