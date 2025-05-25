fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    if *n == 2 {
        return true;
    }
    if *n % 2 == 0 {
        return false;
    }

    let sqrt = (*n as f64).sqrt() as u32;
    for i in (3..=sqrt).step_by(2) {
        if *n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 100, 10007];
    for n in numbers {
        println!("Is {} prime? {}", n, is_prime(&n));
    }
}

#[test]
fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    test_data
        .iter()
        .for_each(|(n, prime)| assert_eq!(is_prime(n), *prime));
}
