// Функція: Чи можливо рівномірно розподілити вантаж
pub fn is_balanced_possible(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    total as usize % shipments.len() == 0
}

// Функція: Підрахунок мінімальних переміщень для вирівнювання
pub fn count_permutation(shipments: &Vec<u32>) -> usize {
    let n = shipments.len();
    let sum: u32 = shipments.iter().sum();
    if sum as usize % n != 0 {
        return 0; // або panic!("Cannot balance")
    }

    let average = sum / n as u32;
    let mut moves = 0;
    let mut diff: i32 = 0;

    for &load in shipments.iter() {
        diff += load as i32 - average as i32;
        moves += diff.abs() as usize;
    }

    moves
}

// Функція: Генерація розподіленого вектора (сума % n == 0)
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![5; n];
    for i in 0..(n / 2) {
        shipments[i] += i as u32;
        shipments[n - 1 - i] -= i as u32;
    }
    shipments
}

// Для перевірки
fn main() {
    let v1 = vec![8, 2, 2, 4, 4];
    println!("{:?} => moves: {}", v1, count_permutation(&v1));

    let v2 = vec![9, 3, 7, 2, 9];
    println!("{:?} => moves: {}", v2, count_permutation(&v2));

    let generated = gen_shipments(5);
    println!("Generated: {:?} => moves: {}", generated, count_permutation(&generated));
}
