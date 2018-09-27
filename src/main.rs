use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::i32;
use std::usize;
use std::cmp::Ordering;

#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // If usize is smaller than i32, some logic is lost

    // Open file
    let filename = env::args().nth(1).expect("no arguments");
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");

    // Read file in
    let mut lines = contents.lines();

    // Parse file
    let count: i32 = lines.next().expect("0 elems").parse().expect("invalid number");
    let mut points = Vec::new();
    if count < 0 {
        panic!("Negative count")
    }

    for point in lines.take(count as usize).map(|x| x.split(" ").collect::<Vec<_>>()) {
        points.push(Point { x: point[0].parse().expect("invalid number"), y: point[1].parse().expect("invalid number") });
    }

    // Sort by x
    points.sort_unstable_by(|x, y| x.x.cmp(&y.x));

    let mut iter = points.iter();

    // Create a stack and push the first two points onto it
    let mut top_stack = Vec::new();
    top_stack.push(iter.next().expect("points empty?"));
    top_stack.push(iter.next().expect("points empty?"));

    // While left turn or only two points, pop stack
    for thing in iter {
        while top_stack.len() >= 2 && Ordering::Greater != det(&thing, top_stack.last().expect("bad"), top_stack.get(top_stack.len() - 2).expect("bad")) {
            top_stack.pop();
        }
        // Push if valid
        top_stack.push(thing);
    }

    // Reverse and repeat for the bottom hull
    let mut iter = points.iter().rev();

    let mut bottom_stack = Vec::new();
    bottom_stack.push(iter.next().expect("points empty?"));
    bottom_stack.push(iter.next().expect("points empty?"));

    // While left turn or only two points, pop stack
    for thing in iter {
        while bottom_stack.len() >= 2 && Ordering::Greater != det(&thing, bottom_stack.last().expect("bad"), bottom_stack.get(bottom_stack.len() - 2).expect("bad")) {
            bottom_stack.pop();
        }
        // Push if valid
        bottom_stack.push(thing);
    }

    // Pull off the first and last, because these are in both
    top_stack.pop();
    top_stack.remove(0);

    // Link the two together
    top_stack.append(&mut bottom_stack);

    // Reverse for counter-clockwise and rename
    top_stack.reverse();
    let solution = top_stack;

    // Output
    let mut out = File::create("output.txt").expect("Couldn't open file for writing");
    println!("{}", solution.len());
    writeln!(out, "{}", solution.len());
    for i in solution {
        println!("{} {}", i.x, i.y);
        writeln!(out, "{} {}", i.x, i.y);
    }
}

// Use det trick to find where a point lies compared to a line
fn det(p: &Point, q: &Point, r: &Point) -> Ordering {
    let val = (q.x * r.y - q.y * r.x) - p.x * (r.y - q.y) + p.y * (r.x - q.x);
    if val > 0 {
        Ordering::Greater
    }
    else if val < 0 {
        Ordering::Less
    }
    else {
        Ordering::Equal
    }
}
