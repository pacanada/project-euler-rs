const FIRST_TWENTY_NAMES: [&str; 20] = [
    "",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS_NAMES: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

struct Number {
    n: u16,
}
impl Number {
    fn new(n: u16) -> Self {
        if n > 9999 {
            unimplemented!()
        }
        Self { n }
    }
    fn format(self) -> String {
        let mut name = String::new();
        // get the thousands
        let thousands = self.n / 1000;
        let mut remainder = self.n - thousands * 1000;
        if thousands > 0 {
            name.push_str(FIRST_TWENTY_NAMES[thousands as usize]);
            name.push_str(" thousand");
        }

        // get the hundreds
        let hundreds = remainder / 100;
        if hundreds > 0 {
            if name.contains("thousand") {
                name.push_str(" and ");
            }
            name.push_str(FIRST_TWENTY_NAMES[hundreds as usize]);
            name.push_str(" hundred");
        }
        remainder -= hundreds * 100;

        // get the tens smaller than 20 or
        if (name.contains("hundred")) & (remainder != 0) {
            name.push_str(" and ");
        }
        if remainder < 20 {
            name.push_str(FIRST_TWENTY_NAMES[remainder as usize]);
        } else {
            let tens = remainder / 10;
            remainder -= tens * 10;

            name.push_str(TENS_NAMES[tens as usize]);
            if remainder != 0 {
                name.push_str("-");
            }

            name.push_str(FIRST_TWENTY_NAMES[remainder as usize]);
        }
        name
    }
}

fn approach_1(max_number: u16) -> usize {
    // for i in 1..max_number+1 {
    //     let n = Number::new(i).format();
    //     println!("{}", n);
    //     //println!("{i}: {} {} {}", n, n.replace("-", "").replace(" ", ""), n.replace("-", "").replace(" ", "").len());
    // }
    // 0
    (1..max_number + 1).fold(0, |add, x| {
        add + Number::new(x)
            .format()
            .replace("-", "")
            .replace(" ", "")
            .len()
    })
}

fn main() {
    let max_number = 1_000;
    let sol = approach_1(max_number);
    println!("The total sum of chars contained in the names from 1 to {max_number} is {sol}");
}
