fn gen_random_vector(_n: usize) -> Vec<i32> {
    // Імітація рандомного вектора (20 чисел від 10 до 99)
    vec![42, 88, 15, 67, 23, 90, 11, 39, 55, 21, 62, 44, 28, 36, 95, 19, 49, 33, 70, 12]
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_index + 1, min_sum)
}

fn print_vector_with_min_sum(data: &[i32]) {
    println!(
        "indexes: {}",
        (0..data.len())
            .map(|i| format!("{:>3}", i))
            .collect::<Vec<_>>()
            .join(".")
    );

    println!(
        "data:    [{}]",
        data.iter()
            .map(|v| format!("{:>2}", v))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let (i, j, sum) = min_adjacent_sum(data);

    let arrows = (0..data.len())
        .map(|idx| {
            if idx == i {
                " \\__".to_string()
            } else if idx == j {
                "__/ ".to_string()
            } else {
                "    ".to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("");

    println!("indexes:{}", arrows);
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[i], data[j], sum, i, j
    );
}

fn main() {
    let vec = gen_random_vector(20);
    print_vector_with_min_sum(&vec);
}
