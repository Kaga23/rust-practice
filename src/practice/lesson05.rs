fn greatest_common_divisor(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let x = 48;
    let y = 18;

    println!("GCD({}, {}) = {}", x, y, greatest_common_divisor(x, y));
}
