fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;
    let mut perm = digits.to_vec();

    permute(&mut perm, 0, &mut count);
    println!("Total solutions: {}", count);
}

fn permute(arr: &mut Vec<u32>, index: usize, count: &mut u32) {
    if index == arr.len() {
        let m = arr[0];
        let u = arr[1];
        let x = arr[2];
        let a = arr[3];
        let s = arr[4];
        let l = arr[5];
        let o = arr[6];
        let n = arr[7];

        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        if muxa * a == slon {
            println!("  {}{}{}{}", m, u, x, a);
            println!("x    {}", a);
            println!("------");
            println!(" {}{}{}{}", s, l, o, n);
            println!();
            *count += 1;
        }

        return;
    }

    for i in index..arr.len() {
        arr.swap(index, i);
        permute(arr, index + 1, count);
        arr.swap(index, i); // backtrack
    }
}
