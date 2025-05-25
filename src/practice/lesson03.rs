const W: usize = 30;
const H: usize = 15;

fn main() {
    let mut result = String::new();

    for y in 0..H {
        for x in 0..W {
            let ch = if y == 0 || y == H - 1 || x == 0 || x == W - 1 {
                '*' // рамка
            } else if x == y * W / H || x == (H - 1 - y) * W / H {
                '*' // діагоналі
            } else {
                ' '
            };
            result.push(ch);
        }
        result.push('\n');
    }

    print!("{}", result);
}
