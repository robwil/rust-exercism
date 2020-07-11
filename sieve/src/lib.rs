use std::collections::HashSet;
use std::convert::TryInto;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let nums: Vec<u64> = (2 ..= upper_bound).collect();
    let mut not_prime: HashSet<u64> = HashSet::new();
    let mut primes: Vec<u64> = Vec::new();
    let mut i = 0;
    while i < nums.len() {
        let x = *nums.get(i).unwrap();
        eprintln!("considering {}", x);
        if not_prime.contains(&x) == false {
            primes.push(x);
        }
        let mut j = i;
        while j < nums.len() {
            not_prime.insert(*nums.get(j).unwrap());
            let x_usize: usize = x.try_into().unwrap();
            j += x_usize;
        }
        i += 1;
    }
    return primes;
}
