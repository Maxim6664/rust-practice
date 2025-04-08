pub fn draw_tree(triangles: usize) {
    for t in 1..=triangles {
        for row in 1..=t + 1 {
            let stars = 2 * row - 1;
            let spaces = triangles + 1 - row;
            let line = " ".repeat(spaces) + &"*".repeat(stars);
            println!("{}", line);
        }
    }
}

fn main() {
    draw_tree(4); // ← кількість трикутників
}
