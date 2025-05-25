fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    let permutations = generate_permutations(&digits, 8);

    for p in permutations {
        let (m, u, x, a, s, l, o, n) = (p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7]);

        let muxa = to_number(&[m, u, x, a]);
        let slon = to_number(&[s, l, o, n]);

        if muxa * a == slon {
            print_result(m, u, x, a, s, l, o, n);
            count += 1;
        }
    }

    println!("Кількість розв'язків: {}", count);
}

/// Генерує всі перестановки заданої довжини без повторів
fn generate_permutations(arr: &[i32], len: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut current = Vec::new();
    let mut used = vec![false; arr.len()];
    backtrack(arr, len, &mut current, &mut used, &mut result);
    result
}

fn backtrack(
    arr: &[i32],
    len: usize,
    current: &mut Vec<i32>,
    used: &mut Vec<bool>,
    result: &mut Vec<Vec<i32>>,
) {
    if current.len() == len {
        result.push(current.clone());
        return;
    }

    for i in 0..arr.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        current.push(arr[i]);
        backtrack(arr, len, current, used, result);
        current.pop();
        used[i] = false;
    }
}

/// Перетворює масив цифр у число
fn to_number(digits: &[i32]) -> i32 {
    digits.iter().fold(0, |acc, &d| acc * 10 + d)
}

/// Виводить форматований результат
fn print_result(m: i32, u: i32, x: i32, a: i32, s: i32, l: i32, o: i32, n: i32) {
    println!("  {}{}{}{}", m, u, x, a);
    println!("×      {}", a);
    println!("--------");
    println!("  {}{}{}{}", s, l, o, n);
    println!();
}
