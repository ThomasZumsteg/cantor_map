use std::ops::Add;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {x: self.x - other.x, y: self.y - other.y}
    }    
}
