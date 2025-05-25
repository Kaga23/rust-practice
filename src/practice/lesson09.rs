fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let shift = ((n % len as isize + len as isize) % len as isize) as usize;

    let (left, right) = s.split_at(len - shift);
    format!("{}{}", right, left)
}

fn main() {
    let s = "abcdefgh".to_string();
    let shifts = [0, 1, 2, -1, -2, 8, -8, 10, -10];

    for n in shifts {
        let rotated = rotate(s.clone(), n);
        println!("rotate({}, {}) = {}", s, n, rotated);
    }
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)| assert_eq!(rotate(s.to_string(), *n), exp.to_string()));
}
