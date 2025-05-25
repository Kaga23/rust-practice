fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let numbers = [123, 121, 1221, 1, 0, 12321];
    for &n in &numbers {
        println!("Is {} a palindrome? {}", n, is_palindrome(n));
    }
}

#[test]
fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    data
        .iter()
        .for_each(|(n, exp)| assert_eq!(is_palindrome(*n), *exp));
}
