use std::collections::HashMap;

// <p>$2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.</p>
// <p>What is the smallest positive number that is <strong class="tooltip">evenly divisible<span class="tooltiptext">divisible with no remainder</span></strong> by all of the numbers from $1$ to $20$?</p>
use project_euler::utils;
fn convert_to_factor_map(vec: Vec<u32>)-> HashMap<u32,u32> {
    let mut factor_track = HashMap::<u32,u32>::new();
    for i in &vec {
        match factor_track.get(&i) {
            Some(v) => {factor_track.insert(*i, v+1);},
            None => {factor_track.insert(*i, 1);}
        }
    }
    return factor_track

}
fn merge(set_1: HashMap<u32,u32>, set_2: HashMap<u32,u32>)->HashMap<u32,u32>{
    let mut new_set = set_1.clone();
    for (k, v) in set_2 {
        match set_1.get(&k) {
            Some(v1)=> {
                let mut max_v = *v1;
                if v>*v1{
                    max_v = v;

                }
                new_set.insert(k, max_v);
            }
            None => {new_set.insert(k, v);},
        }
    }

    new_set

}
fn approach_1(n: u32) -> u32 {
    // Least Common Multiple approach
    // TODO: seems very complicated approach
    let mut lcm = HashMap::<u32, u32>::new();
    for i in 1..n {
        match utils::get_factors(i) {
            Some(factors) => {lcm = merge(lcm.clone(), convert_to_factor_map(factors))},
            None => println!("No factors found for {}",i),
        }

        

    }
    println!("Map of factors, repeations {:?}", lcm);
    let mut product = 1;
    for (k,v) in lcm {
        product*=k.pow(v)


    }
    product
    }

fn main() {
    let n: u32 =20;
    let solution = approach_1(n)

;
    println!(
        "Smallest common multiple of 1 to {}: {:?}", n, solution
    )
    
}
