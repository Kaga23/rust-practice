const H: usize = 15; 

fn main() {
    let mut result = String::new();
    let mid = H / 2;

    for y in 0..H {
        let dx = if y <= mid { y } else { H - y - 1 };
        let spaces = mid - dx;
        let stars = dx * 2 + 1;

        for _ in 0..spaces {
            result.push(' ');
        }
        for _ in 0..stars {
            result.push('*');
        }
        result.push('\n');
    }

    print!("{}", result);
}
