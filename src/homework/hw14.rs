use std::collections::HashSet;

#[derive(Debug)]
struct Rect {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

fn occupied_area(rects: &[Rect]) -> usize {
    let mut occupied: HashSet<(u32, u32)> = HashSet::new();

    for rect in rects {
        for dx in 0..rect.width {
            for dy in 0..rect.height {
                occupied.insert((rect.x + dx, rect.y + dy));
            }
        }
    }

    occupied.len()
}

fn main() {
    let rects = vec![
        Rect { x: 1, y: 3, width: 4, height: 4 },
        Rect { x: 3, y: 1, width: 4, height: 4 },
    ];
    let area = occupied_area(&rects);
    println!("Occupied area: {}", area);
}
