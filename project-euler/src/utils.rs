
use std::{collections::HashMap, hash::Hash};

use num::{PrimInt, NumCast};

pub fn is_prime<T: PrimInt +std::fmt::Debug>(n: T)-> bool {
    // TODO: does it really need to be as ugly?
    // TODO: use mod 6 approach for faster implementation
    let mut is_prime = true;
    let _2 = T::from(2).expect("Could not convert type use to integer");
    let _1 = T::one();
    let _0 = T::zero();
    let mut i = _2;
    
    if (n == _1) | (n == _0) {
        return false
    }
    if n == _2 {return true}
    loop {
        if i > n/_2+_1 {
            break
        }
        if n%i==_0 {
            is_prime = false;
            break
        }
        i = i + _1;
    }
    is_prime
}
pub fn get_factors<T: PrimInt + NumCast+std::fmt::Debug>(n: T) -> Option<Vec<T>> {
    let mut quotient = n;
    let _0 = T::zero();
    let _1 = T::one();
    let _2 = T::from(2).expect("Could not convert type use to integer");
    let mut divisor = _2;
    let mut factors = Vec::<T>::new();
    if (n==_0) | (n==_1) {
        // no factors for 0 or 1
        return None
    }
    loop {
        if quotient==_1 {
            break
        }
        if (is_prime(divisor)) & (quotient%divisor==_0) {
            quotient = quotient/divisor;
            factors.push(divisor);
        }
        if quotient%divisor!=_0 {
            // only increment if we cannot divide again by the same prime number
            divisor = divisor +_1;
        }

    }

Some(factors)
}

///
/// Convert a vector of type T into a hashmap with the key being T and the value the count of
/// ocurrences
///
pub fn convert_to_count_map<T: PrimInt + NumCast + Hash>(vec: Vec<T>)-> HashMap<T,u32> {
    let mut factor_track = HashMap::<T,u32>::new();
    for i in &vec {
        match factor_track.get(&i) {
            Some(v) => {factor_track.insert(*i, v+1);},
            None => {factor_track.insert(*i, 1);}
        }
    }
    return factor_track
}