// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    //see https://rustwiki.org/zh-CN/reference/patterns.html#identifier-patterns
    //why add a & here
    match &y {
        //why add ref here 
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
