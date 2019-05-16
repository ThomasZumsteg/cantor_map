use cantor::Point;
use std::collections::HashSet;
use std::mem::swap;

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        swap(&mut a, &mut b);
    }
    while b > 0 {
        a = a % b;
        swap(&mut a, &mut b);
    }
    a
}

struct CantorPath {
    current: Point,
    diff: Point,
}

impl CantorPath {
    fn new() -> CantorPath {
        CantorPath {
            current: Point::new(0, 0),
            diff: Point::new(1, -1)
        }
    }
}

impl Iterator for CantorPath {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.current.x <= 0 {
            self.current.y += 1;
            self.diff = Point::new(1, -1);
        } else if self.current.y <= 0 {
            self.current.x += 1;
            self.diff = Point::new(-1, 1);
        } else {
            self.current = self.current + self.diff;
        }
        Some(self.current)

    }
}

struct RowColPath {
    start: Point,
    stop: Point,
    current: Point,
}

impl RowColPath {
    fn new(size: usize) -> RowColPath {
        RowColPath {
            current: Point::new(0, 0),
            start: Point::new(0, 0),
            stop: Point::new(size as i32, size as i32)
        }
    }
}

impl Iterator for RowColPath {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let mut pos = self.current;
        if pos.y > self.stop.y {
            return None
        } else if pos.x > self.stop.x {
            pos.y += 1;
            pos.x = self.start.x;
        } else {
            pos.x += 1;
        }
        self.current = pos;
        Some(pos)
    }
}

fn gcd_method(path: &mut Iterator<Item=Point>) -> HashSet<Point> {
    let mut result = HashSet::new();
    for point in path {
        if gcd(point.x as usize, point.y as usize) != 1 {
            result.insert(point);                
        }
    }
    result
}

#[derive(Hash, PartialEq, Eq)]
struct Fraction {
    pub numerator: i32,
    pub denominator: i32,
}

impl Fraction {
    fn new(num: i32, denom: i32) -> Fraction {
        let gcd = gcd(num as usize, denom as usize) as i32;
        Fraction {
            numerator: num / gcd,
            denominator: denom / gcd
        }
    }
}

fn history_method(path: &mut Iterator<Item=Point>) -> HashSet<Point> {
    let mut result = HashSet::new();
    let mut seen = HashSet::new();
    for point in path {
        let f = Fraction::new(point.x, point.y);
        if seen.contains(&f) {
            result.insert(point);
        }
        seen.insert(f);
    }
    result
}

fn draw_svg(filename: &str, points: &HashSet<Point>) {
    let mut document = Document::new();
    for point in points {
        let rect = Data::new()
            .move_to((point.x, point.y))
            .line_by((0, 1))
            .line_by((1, 0))
            .line_by((0, -1))
            .line_by((-1, 0));
        document = document.add(Path::new()
            .set("fill", "black")
            .set("d", rect));
    }
    svg::save(filename, &document).unwrap();
}

fn main() {
    let points = history_method(&mut RowColPath::new(100));
    draw_svg("sample.svg", &points);
}
