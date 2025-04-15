fn main() {
    let input = "HeLLo WoRLd";
    let result = switch_case(input);
    println!("Original: {}", input);
    println!("Switched: {}", result);
}

pub fn switch_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else if c.is_uppercase() {
                c.to_lowercase().to_string()
            } else {
                c.to_string()
            }
        })
        .collect()
}
