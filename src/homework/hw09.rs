pub fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    if len == 0 {
        return s;
    }

    let n = ((n % len as isize) + len as isize) % len as isize;
    let n = n as usize;

    let rotated = format!("{}{}", &s[n..], &s[..n]);
    rotated
}

fn main() {
    let s = "abcdefgh".to_string();
    let result = rotate(s, 2);
    println!("Result: {}", result); // Виведе "ghabcdef"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10, "cdefghab"),
        ];

        for (n, expected) in shifts {
            assert_eq!(rotate(s.clone(), n), expected.to_string());
        }
    }
}
