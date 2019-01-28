pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut primes = PrimeChecker::new();
    let mut i = 1;
    let mut current = n;

    match (1..)
        .filter(|c| primes.check(2 * c + 1))
        .nth((n - 1) as usize)
    {
        None => panic!("oops"),
        Some(x) => 2 * x + 1,
    }
    // while current > 0 {
    //     i = i + 2;
    //     if primes.check(i) {
    //         current = current - 1;
    //     }
    // }

    //    i
}

struct PrimeChecker {
    primes: Vec<u32>,
}

impl PrimeChecker {
    fn new() -> PrimeChecker {
        PrimeChecker { primes: Vec::new() }
    }

    fn check(&mut self, n: u32) -> bool {
        let mut nn = n;

        for i in self.primes.iter() {
            while nn % i == 0 {
                nn = nn / i;
            }
            if nn == 1 {
                return false;
            }
        }
        self.primes.insert(0, n);
        return true;
    }
}

// pub fn is_prime(n: u32) -> bool {
//     !(2..n - 1).any(|i| n % i == 0)
// }

// pub fn nth(n: u32) -> u32 {
//     match n {
//         n if n == 0 => 2,
//         n => match (1..).filter(|c| is_prime(*c * 2 + 1)).nth((n - 1) as usize) {
//             None => panic!("oops"),
//             Some(x) => x * 2 + 1,
//         },
//     }
// }
