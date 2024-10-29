// <p>A Pythagorean triplet is a set of three natural numbers, $a \lt b \lt c$, for which,
// $$a^2 + b^2 = c^2.$$</p>
// <p>For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.</p>
// <p>There exists exactly one Pythagorean triplet for which $a + b + c = 1000$.<br>Find the product $abc$.</p>



    // a^2+b^2 = c^2
    // a+b+c = 1000
    // a<b<c
    // Doing some simplification 
    //      a^2+b^2 = (1000-a-b)(1000-a-b)
    //      a^2+b^2 = 1000000-1000*a-1000*b-1000*a+a^2+a*b-1000*b+a*b+b^2
    //      0       = 1000000-2000*a-2000*b+2*a*b
    // Simplifying for a
    //      2000a+2000b-2ab = 1000000 
    //      a(2000-2b) = 1000000-2000b
    //      a = (1000000-2000b)/(2000-2b) = 2000(500-b)/(2000-2b)
    //      a = 1000(500-a)/(1000-b)
fn approach_1() -> Option<u32> {
    fn b_f(a: f32) -> f32 {
        1000.0*(500.0-a)/(1000.0-a)
    }

    // a cannot be greater than 500
    for a in 1..500 {
        let b = b_f(a as f32);
        println!("{a} : {b}");
        // check that is natural number
        if b.floor()==b {
            return Some(a*(b as u32)*(1000-a-(b as u32)))

        }
    }
    None
 
}



fn main() {

    match approach_1() {
        Some(res) => println!("Product of a*b*c: {res}"),
        None => println!("Could not find solution")
    }
}
