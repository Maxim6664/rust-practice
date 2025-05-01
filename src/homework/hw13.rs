use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn area_occupied(rects: &Vec<Rectangle>) -> i32 {
    let mut occupied: HashSet<(i32, i32)> = HashSet::new();

    for rect in rects {
        let x1 = rect.p1.x.min(rect.p2.x);
        let x2 = rect.p1.x.max(rect.p2.x);
        let y1 = rect.p1.y.min(rect.p2.y);
        let y2 = rect.p1.y.max(rect.p2.y);

        for x in x1..x2 {
            for y in y1..y2 {
                occupied.insert((x, y));
            }
        }
    }

    occupied.len() as i32
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            p1: Point { x: 1, y: 1 },
            p2: Point { x: 4, y: 9 },
        },
        Rectangle {
            p1: Point { x: 3, y: 2 },
            p2: Point { x: 11, y: 8 },
        },
        Rectangle {
            p1: Point { x: 9, y: 7 },
            p2: Point { x: 13, y: 2 },
        },
    ]
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Occupied area: {}", occupied); // Очікувано: 60
}
