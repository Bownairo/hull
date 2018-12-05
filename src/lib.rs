extern crate webgeo_output;

use webgeo_output::*;
use std::i32;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn run(input: Vec<(i32, i32)>, output: &mut Output) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    for point in input {
        points.push(Point { x: point.0, y: point.1 });
    }

    // Remove duplicate points
    let dedup: HashSet<_> = points.drain(..).collect();
    points.extend(dedup.into_iter());

    // Sort by x
    points.sort_unstable_by(|x, y| x.x.cmp(&y.x));

    for point in points.iter() {
        output.points_add(point.x, point.y);
    }

    let mut iter = points.iter();

    // Create a stack and push the first two points onto it
    let mut top_stack = Vec::new();
    let a = iter.next().expect("points empty");
    let b = iter.next().expect("points empty");
    top_stack.push(a);
    top_stack.push(b);
    output.segs_add(a.x, a.y, b.x, b.y);

    // While left turn or only two points, pop stack
    for thing in iter {
        while top_stack.len() >= 2 && Ordering::Less != det(&thing, top_stack.last().expect("bad"), top_stack.get(top_stack.len() - 2).expect("bad"), output) {
            top_stack.pop();
            output.segs_pop();
        }
        // Push if valid
        let x = top_stack.last().unwrap().x;
        let y = top_stack.last().unwrap().y;
        top_stack.push(thing);
        output.segs_add(x, y, thing.x, thing.y);
    }

    // Reverse and repeat for the bottom hull
    let mut iter = points.iter().rev();

    let mut bottom_stack = Vec::new();
    let a = iter.next().expect("points empty");
    let b = iter.next().expect("points empty");
    bottom_stack.push(a);
    bottom_stack.push(b);
    output.segs_add(a.x, a.y, b.x, b.y);

    // While left turn or only two points, pop stack
    for thing in iter {
        while bottom_stack.len() >= 2 && Ordering::Less != det(&thing, bottom_stack.last().expect("bad"), bottom_stack.get(bottom_stack.len() - 2).expect("bad"), output) {
            bottom_stack.pop();
            output.segs_pop();
        }
        // Push if valid
        let x = bottom_stack.last().unwrap().x;
        let y = bottom_stack.last().unwrap().y;
        bottom_stack.push(thing);
        output.segs_add(x, y, thing.x, thing.y);
    }

    // Pull off the first and last, because these are in both
    top_stack.pop();
    top_stack.remove(0);

    // Link the two together
    top_stack.append(&mut bottom_stack);

    let solution = top_stack;

    let mut out = Vec::new();

    for val in solution {
        out.push((val.x, val.y));
    }

    out
}

// Use det trick to find where a point lies compared to a line
fn det(p: &Point, q: &Point, r: &Point, output: &mut Output) -> Ordering {
    output.rays_add(q.x, q.y, p.x, p.y);
    output.rays_pop();
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
