fn count_permutation(shipments: &Vec<u32>) -> isize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return -1; // Неможливо розподілити рівномірно
    }

    let average = (total / n) as i32;
    let mut moves = 0;
    let mut balance = 0;

    for &load in shipments {
        balance += load as i32 - average;
        moves += balance.abs();
    }

    moves as isize
}

/// Функція для генерації розподілюваного набору вантажу
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut result = vec![0; n];
    let average = 10;

    for i in 0..n {
        result[i] = average;
    }

    result
}

fn main() {
    let examples = vec![
        vec![1, 1, 1, 1, 6],              // Sample: Output 4
        vec![8, 2, 2, 4, 4],              // Output 4
        vec![9, 3, 7, 2, 9],              // Output 7
        vec![5, 5, 5, 5],                 // Output 0
        vec![10, 1, 1, 1],                // Impossible => -1
    ];

    for (i, shipments) in examples.iter().enumerate() {
        let result = count_permutation(shipments);
        println!("Example {}: {:?} => moves = {}", i + 1, shipments, result);
    }

    // Демонстрація генерації
    let generated = gen_shipments(6);
    println!("Generated valid input: {:?}", generated);
    println!("Moves required: {}", count_permutation(&generated));
}
