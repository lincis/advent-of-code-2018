use std::io::Write;
use std::convert::TryInto;
use aoc::utils::*;
use ndarray::{Array2, Axis};
// use std::{thread, time};
// use std::io::{Write, stdout};
// use crossterm::{QueueableCommand, cursor};
use std::fs::File;
// use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl From<&std::vec::Vec<i32>> for Point {
    fn from(v: &std::vec::Vec<i32>) -> Point {
        if v.len() < 4 {
            panic!("Source vector has less than 4 elements");
        }
        Point{x: v[0], y: v[1], vx: v[2], vy: v[3]}
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.vx == other.vx && self.vy == other.vy
    }
}

#[derive(Debug)]
struct Dimensions {
    width: usize,
    height: usize,
    shift_x: i32,
    shift_y: i32,
}

impl PartialEq for Dimensions {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height && self.shift_x == other.shift_x && self.shift_y == other.shift_y
    }
}

fn get_dimensions (point_defs: &Vec<Point>) -> Dimensions {
    let mut max_x: i32 = i32::MIN;
    let mut min_x: i32 = i32::MAX;
    let mut max_y: i32 = i32::MIN;
    let mut min_y: i32 = i32::MAX;
    for p in point_defs {
        if max_x < p.x { max_x = p.x; }
        if min_x > p.x { min_x = p.x; }
        if max_y < p.y { max_y = p.y; }
        if min_y > p.y { min_y = p.y; }
    }
    Dimensions{width: (max_x - min_x + 1).try_into().unwrap(), height: (max_y - min_y + 1).try_into().unwrap(), shift_x: min_x, shift_y: min_y}
}

fn do_move (mut point: Point) -> Point {
    point.x += point.vx;
    point.y += point.vy;
    point
}

fn format_points (points: &Vec<Point>) -> String {
    let dimensions = get_dimensions(points);
    let mut point_map: Array2<i32> = Array2::zeros((dimensions.width, dimensions.height));
    for p in points {
        let x: usize = (p.x - dimensions.shift_x).try_into().unwrap();
        let y: usize = (p.y - dimensions.shift_y).try_into().unwrap();
        point_map[[x, y]] = 4;
    }
    point_map.axis_iter(Axis(1)).map(|row| row.iter().map(|&x| if x == 0 {" ".to_string()} else {"X".to_string()}).collect::<String>()).collect::<Vec<String>>().join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dimensions() {
        assert_eq!(
            get_dimensions(&vec![Point{x: 12, y: -3, vx: 0, vy: 0}, Point{x: 1, y: 5, vx: 0, vy: 0}, Point{x: 9, y: 11, vx: 0, vy: 0}]),
            Dimensions{width: 12, height: 15, shift_x: 1, shift_y: -3}
        )
    }
    #[test]
    fn test_move() {
        assert_eq!(do_move(Point{x: 12, y: -5, vx: 2, vy: 5}), Point{x: 14, y: 0, vx: 2, vy: 5});
    }
}

fn main() {
    let mut point_defs: Vec<_> = parse_string_vec::<i32>(&read_inputs("inputs/day-10.txt"), r"position=< *(-*\d+), *(-*\d+)> velocity=< *(-*\d+), *(-*\d+)>").
        iter().map(|p| Point::from(p)).collect();
    let mut dimensions: Vec<Dimensions> = Vec::new();
    let iters: usize = 12000;
    dimensions.reserve(iters);
    let mut file = File::create("output/day-10.txt").unwrap();
    for i in 0 .. iters {
        dimensions.push(get_dimensions(&point_defs));
        point_defs = point_defs.iter().map(|&p| do_move(p)).collect();
        if i > 10140 && i < 10150 {
            println!("{}: {:?}", i, dimensions.last().unwrap());
            file.write_all(format_points(&point_defs).as_bytes()).unwrap();
            file.write_all(b"\n\n").unwrap();
            // match stdout.queue(cursor::SavePosition) {
            //     Ok(_) => (),
            //     Err(_) => (),
            // };
            // match stdout.write(format!("{}: {:?}", i, dimensions.last().unwrap()).as_bytes()) {
            //     Ok(_) => (),
            //     Err(_) => (),
            // };
            // match stdout.write(format!("{:?}", format_points(&point_defs)).as_bytes()) {
            //     Ok(_) => (),
            //     Err(_) => (),
            // };
            // match stdout.queue(cursor::RestorePosition) {
            //     Ok(_) => (),
            //     Err(_) => (),
            // };
            // match stdout.flush() {
            //     Ok(_) => (),
            //     Err(_) => (),
            // };
            // thread::sleep(time::Duration::from_millis(500));
        }
    }
    // println!("Input data = {:?} ({})", point_defs, point_defs.len());
    // let order_def = parse_string_vec::<char>(&input_data, r"Step ([A-Z]) must be finished before step ([A-Z]) can begin.");
    // let mut stdout = stdout();
    //
    // for i in 1..10 {
    //     match stdout.queue(cursor::SavePosition) {
    //         Ok(_) => (),
    //         Err(_) => (),
    //     };
    //     match stdout.write(format!("Here!!! {}", i).as_bytes()) {
    //         Ok(_) => (),
    //         Err(_) => (),
    //     };
    //     match stdout.queue(cursor::RestorePosition) {
    //         Ok(_) => (),
    //         Err(_) => (),
    //     };
    //     match stdout.flush() {
    //         Ok(_) => (),
    //         Err(_) => (),
    //     };
    //     thread::sleep(time::Duration::from_millis(500));
    // }
    //
}
