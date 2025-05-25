fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    data.windows(2)
        .enumerate()
        .map(|(i, pair)| (i, i + 1, pair[0] + pair[1]))
        .min_by_key(|&(_, _, sum)| sum)
        .unwrap()
}

fn print_report(data: &[i32]) {
    let n = data.len();
    let (i, j, sum) = min_adjacent_sum(data);

    // Індекси
    print!("indexes: ");
    for idx in 0..n {
        print!("{:>2}. ", idx);
    }
    println!();

    // Дані
    print!("data:    ");
    for &value in data {
        print!("{:>2}  ", value);
    }
    println!();

    // Стрілки до пари з найменшою сумою
    print!("indexes: ");
    for idx in 0..n {
        if idx == i {
            print!("\\__ ");
        } else if idx == j {
            print!("/  ");
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i], data[j], sum, i, j
    );
    println!();
}

fn main() {
    let test_vectors = vec![
        vec![12, 34, 56, 78, 23, 11, 45, 67, 89, 90, 32, 21, 44, 65, 66, 77, 88, 99, 20, 10],
        vec![91, 82, 73, 64, 55, 46, 37, 28, 19, 10, 11, 22, 33, 44, 55, 66, 77, 88, 99, 13],
        vec![50, 60, 70, 80, 90, 40, 30, 20, 10, 25, 35, 45, 55, 65, 75, 85, 95, 15, 5, 100],
        vec![99, 88, 77, 66, 55, 44, 33, 22, 11, 10, 20, 30, 40, 50, 60, 70, 80, 90, 13, 17],
    ];

    for (i, vec) in test_vectors.iter().enumerate() {
        println!("Test set {}:", i + 1);
        print_report(vec);
    }
}
