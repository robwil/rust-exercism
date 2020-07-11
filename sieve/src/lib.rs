use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    const NOT_PRIME: u64 = 0;
    let mut nums: Vec<u64> = (2..=upper_bound).collect();
    (0..nums.len())
        .filter_map(|i| {
            let prime: u64 = nums[i];
            if prime == NOT_PRIME {
                return None;
            }
            (prime..=upper_bound)
                .step_by(prime as usize)
                .skip(1)
                .for_each(|j| {
                    nums[(j - 2) as usize] = NOT_PRIME;
                });
            Some(prime)
        })
        .collect()
}

pub fn primes_up_to_set(upper_bound: u64) -> Vec<u64> {
    let nums: Vec<u64> = (2..=upper_bound).collect();
    let mut not_prime: HashSet<u64> = HashSet::new();
    (0..nums.len())
        .filter_map(|i| {
            let prime: u64 = (i + 2) as u64;
            (prime..=upper_bound)
                .step_by(prime as usize)
                .skip(1)
                .for_each(|j| {
                    not_prime.insert(j);
                });
            match not_prime.get(&prime) {
                None => Some(prime),
                _ => None,
            }
        })
        .collect()

    // original version:

    // while i < nums.len() {
    //     let x = *nums.get(i).unwrap();
    //     if not_prime.contains(&x) == false {
    //         primes.push(x);
    //     }
    //     let mut j = i;
    //     while j < nums.len() {
    //         not_prime.insert(*nums.get(j).unwrap());
    //         j += x as usize;
    //     }
    //     i += 1;
    // }
    // return primes;
}

// https://exercism.io/tracks/rust/exercises/sieve/solutions/63538fd8918d45759a54b660fc9c7636
pub fn primes_up_to_retain(limit: u32) -> Vec<u32> {
    let mut primes = Vec::new();
    let mut candidates: Vec<_> = (2..limit + 1).rev().collect();
    while let Some(prime) = candidates.pop() {
        primes.push(prime);
        candidates.retain(|n| n % prime != 0);
    }
    primes
}

// https://exercism.io/tracks/rust/exercises/sieve/solutions/5f64e30e37884b9aa001f7f45ca40579
pub fn primes_up_to_starred(upper_bound: u64) -> Vec<u64> {
    let mut numbers: Vec<_> = (2..=upper_bound).map(Option::from).collect();
    (0..numbers.len())
        .filter_map(|i| {
            let prime = numbers[i].take()?;
            (prime..=upper_bound)
                .step_by(prime as usize)
                .skip(1)
                .for_each(|i| numbers[(i - 2) as usize] = None);
            Some(prime)
        })
        .collect()
}
