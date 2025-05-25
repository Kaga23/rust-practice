fn draw_triangle(level: usize, max_width: usize) {
    for i in 0..=level {
        let stars = 2 * i + 1;
        let spaces = (max_width - stars) / 2;
        let line: String = std::iter::repeat(' ')
            .take(spaces)
            .chain(std::iter::repeat('*').take(stars))
            .collect();
        println!("{}", line);
    }
}

fn draw_tree(triangles: usize) {
    // Считаем общее количество строк, чтобы определить ширину последнего треугольника
    let total_levels: usize = (0..triangles).map(|i| i + 1).sum();
    let max_width = 2 * (total_levels - 1) + 1;

    for t in 0..triangles {
        let levels = t + 1;
        draw_triangle(levels - 1, max_width);
    }
}

fn main() {
    let triangles = 4; // Можно изменить количество "ярусов"
    draw_tree(triangles);
}
