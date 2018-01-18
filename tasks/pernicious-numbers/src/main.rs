extern crate aks_test_for_primes;

use std::iter::Filter;
use std::ops::RangeFrom;

use aks_test_for_primes::is_prime;

fn pernicious() -> Filter<RangeFrom<usize>, fn(&usize) -> bool> {
    (0..).filter(is_pernicious as fn(&usize) -> bool)
}

fn is_pernicious(n: &usize) -> bool {
    is_prime(n.count_ones())
}

fn main() {
    for i in pernicious().take(25) {
        print!("{} ", i);
    }
    println!();
    for i in (888_888_877..888_888_888).filter(is_pernicious) {
        print!("{} ", i);
    }
}

#[cfg(test)]
mod tests {
    use super::{is_pernicious, pernicious};

    #[test]
    fn pernicious_iter() {
        let exp = &[
            3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 17, 18, 19, 20, 21, 22, 24, 25, 26, 28, 31, 33, 34,
            35, 36,
        ];
        for (act, &exp) in pernicious().zip(exp.iter()) {
            assert_eq!(act, exp);
        }
    }

    #[test]
    fn is_pernicious_range() {
        let exp = &[
            888888877, 888888878, 888888880, 888888883, 888888885, 888888886
        ];
        for (act, &exp) in (888_888_877..888_888_888)
            .filter(is_pernicious)
            .zip(exp.iter())
        {
            assert_eq!(act, exp);
        }
    }
}
