use std::collections::HashMap;

// <p>The following iterative sequence is defined for the set of positive integers:</p>
// <ul style="list-style-type:none;">
// <li>$n \to n/2$ ($n$ is even)</li>
// <li>$n \to 3n + 1$ ($n$ is odd)</li></ul>
// <p>Using the rule above and starting with $13$, we generate the following sequence:
// $$13 \to 40 \to 20 \to 10 \to 5 \to 16 \to 8 \to 4 \to 2 \to 1.$$</p>
// <p>It can be seen that this sequence (starting at $13$ and finishing at $1$) contains $10$ terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at $1$.</p>
// <p>Which starting number, under one million, produces the longest chain?</p>
// <p class="note"><b>NOTE:</b> Once the chain starts the terms are allowed to go above one million.</p>
fn approach_1(max_number: u32) -> u32 {
    // good oportunity to use caching, it could be approached as well using a reverse approach, working out from 1 backwards?
    fn get_size_of_collatz_sequence(mut number: u32, collatz_map: &mut HashMap<u32, u32>) -> u32 {
        if collatz_map.contains_key(&number) {
            return *collatz_map.get(&number).unwrap()

        }
        let mut seq_length = 1;

        while number!=1 {
            if number%2 == 0 {
                number /= 2;
            }
            else {
                number=number*3+1;
            }
            seq_length+=1;
            collatz_map.insert(number, seq_length);
        }
        
        seq_length
        
    }
    let mut collatz_map: HashMap<u32, u32> = HashMap::new();
    let mut max_seq = 0;
    for i in 1..max_number {
        let seq = get_size_of_collatz_sequence(i, &mut collatz_map);
        if seq>max_seq {
            max_seq = seq;
            println!("Number: {i}, collatz seq {seq}");
        }
    }
    max_seq

    
}

fn main() {

    let max_number = 1_000_000;
    let sol = approach_1(max_number);
    println!("The number under {max_number} that produces the longest collatz sequence is {sol}");
}
