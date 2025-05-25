use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point, // верхня ліва
    b: Point, // нижня права
}

/// Функція, яка рахує фактично зайняту площу
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut occupied = HashSet::new();

    for rect in xs {
        let x_min = rect.a.x.min(rect.b.x);
        let x_max = rect.a.x.max(rect.b.x);
        let y_min = rect.a.y.min(rect.b.y);
        let y_max = rect.a.y.max(rect.b.y);

        for x in x_min..x_max {
            for y in y_min..y_max {
                occupied.insert((x, y));
            }
        }
    }

    occupied.len() as i32
}

/// Прикладові тестові дані з прикладу
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 5, y: 7 },
            b: Point { x: 11, y: 8 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 50);
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Occupied area: {}", occupied);
}
