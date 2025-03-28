const WIDTH: usize = 5;
const HEIGHT: usize = 3;

fn main() {
    let mut output = String::new();
    let total_height = HEIGHT * 2 + 1;
    let total_width = WIDTH * 2 + 1;

    for i in 0..total_height {
        for j in 0..total_width {
            if i + j == WIDTH || j == WIDTH - i + total_height - 1 || (i == HEIGHT && j % 2 == 0) {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
